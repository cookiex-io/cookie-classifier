use std::{collections::HashMap, sync::Arc};
use axum::{http::StatusCode, Extension, Json};
use futures::future::join_all;
use rig::{agent::Agent, providers::openai::CompletionModel};
use crate::{dto::cookie_dto::{CookieResponse, CookiesClassificationRequest}, model::open_list::{CookieCategory, OpenCookie, OpenTracker}, service::classifier::LLMClassification};

pub async fn handle_cookie_classify_request(
    Extension(open_cookies_cache):Extension<Arc<HashMap<String,OpenCookie>>>,
    Extension(open_trackers_cache):Extension<Arc<HashMap<String,OpenTracker>>>,
    Extension(openai_agent): Extension<Arc<Agent<CompletionModel>>>,
    Json(req):Json<CookiesClassificationRequest>
) -> Result<(StatusCode,Json<HashMap<String,CookieResponse>>),(StatusCode,&'static str)>{
    let mut cookies_response:HashMap<String,CookieResponse> = HashMap::new();
    let mut prompt_api_calls = vec![];

    for cookie in req.cookies{
      let cookie_classified = cookie.classify_via_cache(&open_cookies_cache, &open_trackers_cache);
      if cookie_classified.category != CookieCategory::Unclassified{
        cookies_response.insert(cookie.name, cookie_classified);
      }else{
         prompt_api_calls.push(openai_agent.classify_cookie(cookie));
      }
    }
    let api_responses = join_all(prompt_api_calls).await;
    for api_response in api_responses{
       let (name,cookie) = api_response
         .map_err(|e|{
          println!("{}",e);
         (StatusCode::SERVICE_UNAVAILABLE,"Prompt classifiy failed")})?;
       cookies_response.insert(name, cookie);
    }
  Ok((StatusCode::OK,Json(cookies_response)))   
}