use std::{sync::Arc, time::Duration};
use anyhow::Error;
use api::handler::handle_cookie_classify_request;
use axum::routing::{get, post};
use axum::{BoxError, Json, Router};
use axum::{error_handling::HandleErrorLayer, Extension,http::StatusCode};
use infrastructure::cache::REDIS_URI;
use mongodb::Client;
use serde_json::{json, Value};
use service::layer::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod infrastructure;
mod service;
mod model;
mod utils;
mod dto;
mod api;

async fn root(Extension(client): Extension<Arc<Client>>) -> Json<Value> {
    for db_name in client.list_database_names().await.unwrap() {
        println!("{}", db_name);
    }
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client,open_cookies_cache,open_trackers_cache) = match infrastructure::server::initialize_db_client_and_cache().await {
        Ok(client) => client,
        Err(e) => panic!("Failed to initialize database client: {}", e),
    };
    let redis = redis::Client::open(REDIS_URI.as_str()).unwrap();
    let redis_connection = redis.get_multiplexed_async_connection().await.unwrap();
    let make_service = |key: &str, rate: u32, window: Duration| {
        let ratelimit = RateLimitLayer::new(
            rate,
            window,
            redis_connection.clone(),
            format!("ratelimit_{key}"),
        );
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::TOO_MANY_REQUESTS
            }))
            .layer(ratelimit)
    };
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_credentials(false);
    let app = Router::new()
        .route("/", get(root))
        .route("/api/cookies/classify",post(handle_cookie_classify_request).layer(make_service("api/cookies/classify", 5, Duration::from_secs(60))))
        .layer(Extension(client))
        .layer(Extension(open_cookies_cache))
        .layer(Extension(open_trackers_cache))
        .layer(cors);
    let port: u16 = 3000;
    let bind_address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}