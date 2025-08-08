//! Authentication handlers for Slack OAuth and JWT token issuance.
//! Provides endpoints for starting the OAuth flow and handling the callback,
//! including token creation and cookie management.

use std::collections::BTreeMap;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use cookie::time::Duration;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use reqwest::Client;
use serde::Deserialize;
use sha2::Sha256;
use crate::api::errors::AppError;
use crate::api::routes::{RouterState, FORBIDDEN_MSG};

/// Query parameters for the OAuth callback.
#[derive(Deserialize)]
pub struct AuthCallback {
    code: String,
}

/// Initiates the Slack OAuth authentication flow.
/// Redirects the user to Slack's authorization URL.
///
/// # Parameters
/// - `state`: Shared application state containing environment variables.
///
/// # Returns
/// A redirect response to Slack's OAuth page.
pub async fn auth(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Response), AppError> {
    let scopes = "im:read";
    let slack_auth_url = format!(
        "https://slack.com/oauth/v2/authorize?client_id={}&scope={}&redirect_uri={}",
        state.env_vars.slack_client_id, scopes, state.env_vars.slack_redirect_uri
    );

    Ok((
        StatusCode::FOUND,
        Response::builder()
            .header("Location", slack_auth_url)
            .body(Body::empty())
            .unwrap(),
    ))
}

/// Handles the OAuth callback from Slack.
/// Exchanges the code for an access token, verifies the user, and sets a JWT cookie.
///
/// # Parameters
/// - `state`: Shared application state.
/// - `request`: Query parameters containing the OAuth code.
/// - `jar`: Cookie jar for setting authentication cookies.
///
/// # Returns
/// On success, sets a JWT token cookie and redirects to the home page.
/// On failure, returns an unauthorized response.
pub async fn auth_callback(
    State(state): State<RouterState>,
    Query(request): Query<AuthCallback>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar, Response), AppError> {
    let code = request.code;
    let slack_auth_url = format!(
        "https://slack.com/api/oauth.v2.access?client_id={}&client_secret={}&code={}&redirect_uri={}",
        state.env_vars.slack_client_id, state.env_vars.slack_client_secret, code, state.env_vars.slack_redirect_uri
    );
    // Request Slack for access token
    let response = Client::new().get(slack_auth_url).send().await?;

    if response.status() != StatusCode::OK {
        return Ok((
            StatusCode::UNAUTHORIZED,
            jar,
            Body::from(FORBIDDEN_MSG).into_response(),
        ));
    }

    let body = response.text().await?;
    let json_body: serde_json::Value = serde_json::from_str(&body)?;

    let access_token = json_body["access_token"].as_str().unwrap();

    let user_id = json_body["authed_user"]["id"].as_str().unwrap();
    let user = state.tummy.get_user_info(user_id).await?;

    if user.id.is_empty() || user.is_bot || user.deleted {
        return Ok((
            StatusCode::UNAUTHORIZED,
            jar,
            Body::from(FORBIDDEN_MSG).into_response(),
        ));
    }

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(state.env_vars.slack_signing_secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id);
    claims.insert("access_token", access_token);
    let token_str = claims.sign_with_key(&key)?;

    let token_cookie = Cookie::build(("token", token_str))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(state.env_vars.keep_logged_in_for_days));

    Ok((
        StatusCode::PERMANENT_REDIRECT,
        jar.add(token_cookie),
        Response::builder()
            .header("Location", "/".to_owned())
            .body(Body::empty())
            .unwrap(),
    ))
}