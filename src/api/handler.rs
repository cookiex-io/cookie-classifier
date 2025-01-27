use std::{collections::HashMap, sync::Arc};
use axum::{http::StatusCode, Extension, Json};
use crate::{dto::cookie::{CookieResponse, CookiesClassificationRequest}, model::open_list::{OpenCookie, OpenTracker}};

pub async fn handle_cookie_classify_request(
    Extension(open_cookies_cache):Extension<Arc<HashMap<String,OpenCookie>>>,
    Extension(open_trackers_cache):Extension<Arc<HashMap<String,OpenTracker>>>,
    Json(req):Json<CookiesClassificationRequest>
) -> Result<(StatusCode,Json<Vec<CookieResponse>>),(StatusCode,&'static str)>{
    let mut cookies_response_list:Vec<CookieResponse> = Vec::new();
    for cookie in req.cookies{
      cookies_response_list.push(cookie.classify_via_cache(&open_cookies_cache, &open_trackers_cache));
    }
  Ok((StatusCode::OK,Json(cookies_response_list)))   
}