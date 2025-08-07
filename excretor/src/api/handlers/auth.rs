use std::collections::BTreeMap;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
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


#[derive(Deserialize)]
pub struct AuthCallback {
    code: String,
}

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
    // request slack for access token
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

pub async fn login() -> Result<(StatusCode, Response), AppError> {
    todo!();
    // Ok((
    //     StatusCode::OK,
    //     Html(templates::LoginTemplate.render()?).into_response(),
    // ))
}