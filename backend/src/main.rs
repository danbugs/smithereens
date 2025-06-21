use axum::{
    extract::{Path, Query, State},
    http,
    routing::{get, post},
    serve, Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use tracing_subscriber::prelude::*;

mod startgg;
use startgg::{StartggClient, StartggError};

#[derive(Debug, Deserialize)]
struct SearchRequest {
    gamer_tag: String,
    page: Option<u32>,     // optional, defaults to 1
    per_page: Option<u32>, // optional, defaults to 25
}

#[derive(Debug, Deserialize)]
struct ProfilePath {
    slug: String,
}

#[derive(Debug, Deserialize)]
struct ProfileQuery {
    page: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct SetsPath {
    event_slug: String,
}

#[derive(Debug, Deserialize)]
struct SetsQuery {
    user_id: i64,
}

#[derive(Serialize)]
struct ApiError {
    message: String,
}

type ApiResult<T> = Result<Json<T>, (http::StatusCode, Json<ApiError>)>;

#[instrument]
// -------------------------------------------------------------------
// GET /api
// Returns a simple "Hello, world!" message.
// Example cURL:
// curl http://localhost:3000/api
// -------------------------------------------------------------------
async fn hello_handler() -> &'static str {
    "Hello, world!"
}

#[instrument]
// -------------------------------------------------------------------
// POST /api/search
// Uses JSON body: { "gamer_tag": "MkLeo" }
// Example cURL:
// curl -X POST http://localhost:3000/api/search \
//      -H 'content-type: application/json' \
//      -d '{"gamer_tag":"MkLeo"}' | jq
// -------------------------------------------------------------------
async fn search_handler(
    State(client): State<StartggClient>,
    Json(req): Json<SearchRequest>,
) -> ApiResult<serde_json::Value> {
    let gamer_tag = req.gamer_tag.trim().to_string();
    let pg = req.page.unwrap_or(1);
    let pp = req.per_page.unwrap_or(25);

    match client.search_players(&gamer_tag, pg, pp).await {
        Ok(data) => Ok(Json(data)),
        Err(StartggError::RateLimited) => Err((
            http::StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError {
                message: "Rate limit exceeded".into(),
            }),
        )),
        Err(StartggError::GraphQl(msg)) => Err((
            http::StatusCode::BAD_GATEWAY,
            Json(ApiError { message: msg }),
        )),
        Err(StartggError::Upstream(code)) => Err((
            code,
            Json(ApiError {
                message: "Upstream error".into(),
            }),
        )),
        Err(e) => {
            tracing::error!(?e, "internal error");
            Err((
                http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Internal error".into(),
                }),
            ))
        }
    }
}

#[instrument]
// -------------------------------------------------------------------
// GET /api/profile/:slug
// Returns the profile of a player by their slug.
// Example cURL:
// curl http://localhost:3000/api/profile/mkleo
// -------------------------------------------------------------------
async fn profile_handler(
    State(client): State<StartggClient>,
    Path(path): Path<ProfilePath>,
    Query(q): Query<ProfileQuery>,
) -> ApiResult<serde_json::Value> {
    let pg = q.page.unwrap_or(1);
    match client.user_profile(&path.slug, pg).await {
        Ok(data) => Ok(Json(data)),
        Err(StartggError::RateLimited) => Err((
            http::StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError {
                message: "Rate limit exceeded".into(),
            }),
        )),
        Err(StartggError::GraphQl(msg)) => Err((
            http::StatusCode::BAD_GATEWAY,
            Json(ApiError { message: msg }),
        )),
        Err(StartggError::Upstream(code)) => Err((
            code,
            Json(ApiError {
                message: "Upstream error".into(),
            }),
        )),
        Err(e) => {
            tracing::error!(?e, "internal error");
            Err((
                http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Internal error".into(),
                }),
            ))
        }
    }
}

#[instrument]
// -------------------------------------------------------------------
// GET /api/event-sets/:event_slug
// Returns the sets for a player in a specific event.
// Example cURL:
// curl "http://localhost:3000/api/event-sets/tournament%2Fmicrospacing-vancouver-93%2Fevent%2Fultimate-singles?user_id=747662"
// -------------------------------------------------------------------
async fn event_sets_handler(
    State(client): State<StartggClient>,
    Path(path): Path<SetsPath>,
    Query(q): Query<SetsQuery>,
) -> ApiResult<serde_json::Value> {
    match client.player_event_sets(q.user_id, &path.event_slug).await {
        Ok(val) => Ok(Json(val)),
        Err(StartggError::RateLimited) => Err((
            http::StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError {
                message: "Rate limit exceeded".into(),
            }),
        )),
        Err(StartggError::GraphQl(msg)) => Err((
            http::StatusCode::BAD_GATEWAY,
            Json(ApiError { message: msg }),
        )),
        Err(StartggError::Upstream(code)) => Err((
            code,
            Json(ApiError {
                message: "Upstream error".into(),
            }),
        )),
        Err(e) => {
            tracing::error!(?e, "internal error");
            Err((
                http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Internal error".into(),
                }),
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = StartggClient::from_env()?;

    let app = Router::new()
        .route("/api", get(hello_handler))
        .route("/api/search", post(search_handler))
        .route("/api/profile/:slug", get(profile_handler))
        .route("/api/event-sets/:event_slug", get(event_sets_handler))
        .with_state(client.clone());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()?;
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listening on {}", addr);
    serve(listener, app).await?;
    Ok(())
}
