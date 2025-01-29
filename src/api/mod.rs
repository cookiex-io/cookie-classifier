use axum::{middleware::from_extractor, routing::post, Router};
use handler::handle_cookie_classify_request;
use crate::infrastructure::auth::AuthKey;

pub mod handler;

pub fn classify_open_routes() -> Router {
    Router::new()
     .route("/api/open/classify", post(handle_cookie_classify_request))
}


pub fn classify_routes() -> Router {
    Router::new()
     .route("/api/classify", post(handle_cookie_classify_request))
     .route_layer(from_extractor::<AuthKey>())
}
