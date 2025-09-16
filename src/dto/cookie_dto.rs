use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{model::open_list::{CookieCategory, OpenCookie, OpenTracker}, service::classifier::COOKIE_CLASSIFICATION_PROMPT};

#[derive(Serialize, Deserialize, Debug)]
pub struct CookiePromptResponse{
    pub category:CookieCategory,
    pub description:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CookieRequest{
    pub name:String,
    pub provider:String,
}

impl CookieRequest {
    pub fn to_prompt(&self) -> String {
      COOKIE_CLASSIFICATION_PROMPT
       .replace("{name}", &self.name)
       .replace("{provider}", &self.provider)
    }
}

impl CookieRequest {
    pub fn classify_via_cache(&self,open_cookies_cache:&HashMap<String,OpenCookie>,open_trackers_cache:&HashMap<String,OpenTracker>) -> CookieResponse {
        let mut category = CookieCategory::Unclassified;
        let mut description = None;
        if let Some(open_cookie) = open_cookies_cache.get(&self.filtered_name()){
            category = open_cookie.category.clone();
            description = Some(open_cookie.description.clone());
          }else if let Some(open_tracker) = open_trackers_cache.get(&self.provider.replace("www.", "")){
            category = open_tracker.category.clone();
        }
      CookieResponse{provider:self.provider.clone(),category,description}
    }

    pub fn filtered_name(&self,) -> String {
        if self.name.starts_with("_ga_") { // google analytics
            String::from("_ga")
        } else if self.name.starts_with("_fbp_") { // facebook
            String::from("_fbp")
        } else {
           self.name.to_string()
        }
    }  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CookieResponse{
    pub provider:String,
    pub category:CookieCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description:Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CookiesClassificationRequest{
    pub domain:Option<String>,
    pub cookies:Vec<CookieRequest>
}


