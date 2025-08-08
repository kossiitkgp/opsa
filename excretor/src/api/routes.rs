use crate::{db::tummy::Tummy, env::EnvVars};
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use reqwest::Client;
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::api::errors;
use crate::api::handlers;

pub(super) const FORBIDDEN_MSG: &str = "Mortals are forbidden from accessing the site";

/// Verifies a JWT token by checking its validity and then using the
/// embedded access token to call Slack's `auth.test` API.
async fn verify_token(token: &String, state: &RouterState) -> Result<bool, errors::AppError> {
    // verify the jwt token and accessing slack auth test api
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(state.env_vars.slack_signing_secret.as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    let user_id = claims.get("user_id").unwrap();
    let access_token = claims.get("access_token").unwrap();

    let slack_auth_test_url = "https://slack.com/api/auth.test";
    let req = Client::new()
        .get(slack_auth_test_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .build()?;
    let response = Client::new().execute(req).await?;

    if response.status() != StatusCode::OK {
        return Ok(false);
    }

    let user = state.tummy.get_user_info(user_id).await?;
    if user.id.is_empty() || user.is_bot || user.deleted {
        return Ok(false);
    }

    Ok(true)
}

/// A middleware that checks for a valid "token" cookie and verifies it.
async fn verify_token_middleware(
    State(state): State<RouterState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, errors::AppError> {
    if state.env_vars.slack_auth_enable {
        if let Some(token) = jar.get("token").map(|cookie| cookie.value().to_owned()) {
            let is_verified = verify_token(&token, &state).await?;
            if !is_verified {
                return Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from(FORBIDDEN_MSG))
                    .unwrap());
            }
        } else {
            return Ok(Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header("Location", "/login")
                .body(Body::empty())
                .unwrap());
        }
    }

    Ok(next.run(request).await)
}

#[derive(Clone)]
pub(super) struct RouterState {
    pub tummy: Tummy,
    pub env_vars: EnvVars,
}

pub fn get_excretor_router(tummy: Tummy, env_vars: EnvVars) -> Router {
    let state = RouterState { tummy, env_vars };

    let api_router = Router::new()
        .route("/channels", get(handlers::get_channels))
        .route("/users", get(handlers::get_users))
        .route("/channels/:channel_id", get(handlers::load_channel))
        .route("/messages/:channel_id", get(handlers::get_messages))
        .route("/replies", get(handlers::get_replies))
        .route("/search", post(handlers::search));

    Router::new()
        .nest("/api", api_router)
        // The router now calls handler functions from the new `handlers` module.
        .route("/", get(handlers::serve_react_app))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            verify_token_middleware,
        ))
        .route("/auth", get(handlers::auth))
        .route("/auth/callback", get(handlers::auth_callback))
        .route("/assets/*file", get(handlers::assets))
        .with_state(state)
}
