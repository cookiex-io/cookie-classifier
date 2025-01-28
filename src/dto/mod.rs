use axum::{routing::post, Router};
use crate::api::handler::handle_cookie_classify_request;

pub mod cookie;

pub fn classify_open_routes() -> Router {
    Router::new()
     .route("/api/open/classify", post(handle_cookie_classify_request))
}


pub fn classify_routes() -> Router {
    Router::new()
     .route("/api/classify", post(handle_cookie_classify_request))
}
