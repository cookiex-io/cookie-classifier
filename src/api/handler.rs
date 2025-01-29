use std::{collections::HashMap, sync::Arc};
use axum::{http::StatusCode, Extension, Json};
use crate::{dto::cookie::{CookieResponse, CookiesClassificationRequest}, model::open_list::{CookieCategory, OpenCookie, OpenTracker}};

pub async fn handle_cookie_classify_request(
    Extension(open_cookies_cache):Extension<Arc<HashMap<String,OpenCookie>>>,
    Extension(open_trackers_cache):Extension<Arc<HashMap<String,OpenTracker>>>,
    Json(req):Json<CookiesClassificationRequest>
) -> Result<(StatusCode,Json<HashMap<String,CookieResponse>>),(StatusCode,&'static str)>{
    let mut cookies_response:HashMap<String,CookieResponse> = HashMap::new();
    for cookie in req.cookies{
      let cookie_classified = cookie.classify_via_cache(&open_cookies_cache, &open_trackers_cache);
      if cookie_classified.category != CookieCategory::Unclassified{
        cookies_response.insert(cookie.name, cookie_classified);
      }
    }
  Ok((StatusCode::OK,Json(cookies_response)))   
}