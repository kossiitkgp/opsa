use crate::{env::EnvVars, tummy::Tummy};
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use reqwest::Client;
use sha2::Sha256;
use std::collections::BTreeMap;

const FORBIDDEN_MSG: &str = "Mortals are forbidden from accessing the site";

async fn verify_token(token: &String, state: &RouterState) -> Result<bool, handlers::AppError> {
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

async fn verify_token_middleware(
    State(state): State<RouterState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, handlers::AppError> {
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
struct RouterState {
    pub tummy: Tummy,
    pub env_vars: EnvVars,
}

pub fn get_excretor_router(tummy: Tummy, env_vars: EnvVars) -> Router {
    let state = RouterState { tummy, env_vars };

    Router::new()
        // `GET /` goes to `root`
        .route("/", get(handlers::root))
        .route("/channels/:channel_name", get(handlers::load_channel))
        .route("/messages/:channel_id", get(handlers::get_messages))
        .route("/fallback-avatar", get(handlers::fallback_avatar))
        .route("/replies", get(handlers::get_replies))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            verify_token_middleware,
        ))
        .route("/auth", get(handlers::auth))
        .route("/auth/callback", get(handlers::auth_callback))
        .route("/login", get(handlers::login))
        .route("/assets/*file", get(handlers::assets))
        .with_state(state)
}

mod handlers {
    use super::RouterState;
    use super::FORBIDDEN_MSG;
    use crate::templates;
    use crate::tummy::SlackDateTime;
    use askama::Template;
    use axum::extract::{Path, Query, State};
    use axum::response::IntoResponse;
    use axum::{
        body::Body,
        http::StatusCode,
        response::{Html, Response},
    };
    use axum_extra::extract::cookie::{Cookie, CookieJar};
    use cookie::time::Duration;
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use reqwest::Client;
    use serde::Deserialize;
    use sha2::Sha256;
    use sqlx::types::chrono;
    use std::collections::BTreeMap;
    use tokio_util::io::ReaderStream;

    pub(super) struct AppError(color_eyre::eyre::Error);

    impl IntoResponse for AppError {
        fn into_response(self) -> axum::response::Response {
            tracing::error!("An error occured: {}", self.0);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Something went wrong. Please try again later"),
            )
                .into_response()
        }
    }

    #[derive(Deserialize)]
    pub struct ReplyRequest {
        pub channel_id: String,
        pub ts: String,
        pub user_id: String,
    }

    /// Utility function for mapping any error into a `500 Internal Server Error`
    /// response.
    impl<E> From<E> for AppError
    where
        E: Into<color_eyre::eyre::Error>,
    {
        fn from(err: E) -> Self {
            Self(err.into())
        }
    }

    #[derive(Deserialize)]
    pub struct Pagination {
        last_msg_timestamp: Option<String>,
        per_page: u32,
    }

    #[derive(Deserialize)]
    pub struct DateQuery {
        since: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct AuthCallback {
        code: String,
    }

    pub(super) async fn root(
        State(state): State<RouterState>,
    ) -> Result<(StatusCode, Response), AppError> {
        let channels = state.tummy.get_all_channels().await?;

        Ok((
            StatusCode::OK,
            Html(
                templates::IndexTemplate {
                    title: state.env_vars.title,
                    description: state.env_vars.description,
                    channels,
                }
                .render()?,
            )
            .into_response(),
        ))
    }

    pub(super) async fn load_channel(
        State(state): State<RouterState>,
        Path(channel): Path<String>,
    ) -> Result<(StatusCode, Response), AppError> {
        let channel = state.tummy.get_channel_info(&channel).await?;

        Ok((
            StatusCode::OK,
            Html(templates::ChannelTemplate { channel }.render()?).into_response(),
        ))
    }

    pub(super) async fn get_messages(
        State(state): State<RouterState>,
        Path(channel_id): Path<String>,
        pagination: Query<Pagination>,
        date_query: Query<DateQuery>,
    ) -> Result<(StatusCode, Response), AppError> {
        let messages = state
            .tummy
            .fetch_msg_page(
                &channel_id,
                &pagination
                    .last_msg_timestamp
                    .as_ref()
                    .map(|ts| chrono::NaiveDateTime::from_pg_ts(ts)),
                &pagination.per_page,
                &date_query
                    .since
                    .as_ref()
                    .map(|ts| chrono::NaiveDateTime::from_pg_ts(ts))
                    .unwrap_or(chrono::NaiveDateTime::UNIX_EPOCH),
            )
            .await?;

        let new_last_msg_timestamp = messages
            .last()
            .map(|message| message.timestamp)
            .unwrap_or(chrono::NaiveDateTime::UNIX_EPOCH);

        Ok((
            StatusCode::OK,
            Html(
                templates::ChannelPageTemplate {
                    messages,
                    last_msg_timestamp: new_last_msg_timestamp.to_string(),
                    channel_id,
                }
                .render()?,
            )
            .into_response(),
        ))
    }

    pub(super) async fn get_replies(
        State(state): State<RouterState>,
        message_data: Query<ReplyRequest>,
    ) -> Result<(StatusCode, Response), AppError> {
        let messages = state
            .tummy
            .fetch_replies(
                &message_data.ts,
                &message_data.channel_id,
                &message_data.user_id,
            )
            .await?;

        Ok((
            StatusCode::OK,
            Html(
                templates::ThreadTemplate {
                    messages,
                    parent_ts: message_data.ts.clone(),
                    channel_id: message_data.channel_id.clone(),
                    parent_user_id: message_data.user_id.clone(),
                }
                .render()
                .unwrap(),
            )
            .into_response(),
        ))
    }

    pub(super) async fn fallback_avatar() -> Result<(StatusCode, Response), AppError> {
        Ok((
            StatusCode::OK,
            Html(templates::FallbackAvatarTemplate.render()?).into_response(),
        ))
    }

    pub(super) async fn assets(
        State(state): State<RouterState>,
        Path(filepath): Path<String>,
    ) -> Result<(StatusCode, Response), AppError> {
        let final_file_path = state
            .env_vars
            .static_assets_dir
            .join(&filepath)
            .canonicalize()?;

        if final_file_path.starts_with(state.env_vars.static_assets_dir) {
            let file = tokio::fs::File::open(final_file_path).await?;

            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);

            Ok((StatusCode::OK, body.into_response()))
        } else {
            tracing::warn!(
                "A mortal requested to access forbidden file `{}`.",
                filepath
            );

            Ok((
            	StatusCode::FORBIDDEN,
             	Body::from(String::from("Mortals are forbidden from accessing the requested file. This sin will be reported.")).into_response()
            ))
        }
    }

    pub(super) async fn auth(
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

    pub(super) async fn auth_callback(
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

    pub(super) async fn login() -> Result<(StatusCode, Response), AppError> {
        Ok((
            StatusCode::OK,
            Html(templates::LoginTemplate.render()?).into_response(),
        ))
    }
}
