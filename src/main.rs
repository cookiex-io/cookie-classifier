use std::sync::Arc;
use std::time::Duration;
use anyhow::Error;
use axum::routing::get;
use axum::{BoxError, Json, Router};
use axum::{error_handling::HandleErrorLayer, Extension,http::StatusCode};
use api::{classify_open_routes, classify_routes};
use dto::csv_dto::OpenTrackerCsvColumn;
use infrastructure::cache::{REDIS_HOST_NAME, REDIS_PRIMARY_PASSWORD};
use infrastructure::csv::get_list_from_csv;
use mongodb::Client;
use serde_json::{json, Value};
use service::layer::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use lazy_static::lazy_static;
use reqwest::Client as ReqClient;

mod infrastructure;
mod service;
mod model;
mod utils;
mod dto;
mod api;

lazy_static!{
    pub static ref REQUESTS:u32 = std::env::var("REQUESTS").expect("Must set REQUESTS").parse::<u32>().expect("Parsing failed REQUESTS");
    pub static ref TIME:u64 = std::env::var("TIME").expect("Must set TIME").parse::<u64>().expect("Parsing failed TIME");
}

async fn root(Extension(client): Extension<Client>) -> Json<Value> {
    for db_name in client.list_database_names().await.unwrap() {
        println!("{}", db_name);
    }
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let req_client = Arc::new(ReqClient::new());
    let (client,open_cookies_cache,open_trackers_cache) = match infrastructure::server::initialize_db_client_and_cache(&req_client).await {
        Ok(client) => client,
        Err(e) => panic!("Failed to initialize database client: {}", e),
    };
    let redis = redis::Client::open(format!("rediss://:{}@{}",REDIS_PRIMARY_PASSWORD.as_str(),REDIS_HOST_NAME.as_str())).unwrap();
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
        .merge(classify_open_routes().layer(make_service("api/open/classify", *REQUESTS, Duration::from_secs(*TIME))))
        .merge(classify_routes())
        .layer(Extension(client))
        .layer(Extension(req_client))
        .layer(Extension(open_cookies_cache))
        .layer(Extension(open_trackers_cache))
        .layer(cors);
    let port: u16 = 3000;
    let bind_address = format!("0.0.0.0:{}", port);
    println!("Started listening port {port}");
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}