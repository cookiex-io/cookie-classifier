use serde::Deserialize;
use crate::model::open_list::{CookieCategory, OpenCookie, OpenTracker};

#[derive(Deserialize)]
pub enum OpenCookieCategory {
    Functional,
    Personalization,
    Analytics,
    Marketing,
    Security,
}

pub trait ToCookieCategory {
    fn to_cookie_category(&self) -> CookieCategory;
}

impl ToCookieCategory for OpenCookieCategory {
    fn to_cookie_category(&self) -> CookieCategory {
       match &self {
        OpenCookieCategory::Functional => CookieCategory::Necessary,
        OpenCookieCategory::Personalization => CookieCategory::Preference,
        OpenCookieCategory::Analytics => CookieCategory::Statistics,
        OpenCookieCategory::Marketing => CookieCategory::Marketing,
        OpenCookieCategory::Security => CookieCategory::Necessary,
      }
    }
}

#[derive(Deserialize)]
pub struct OpenCookieCsvColumn{
    #[serde(rename = "Category")]
    category:OpenCookieCategory,
    #[serde(rename = "Cookie / Data Key name")]
    pub name:String,
    #[serde(rename = "Description")]
    description:String,
}

impl OpenCookieCsvColumn {
    pub fn to_cookie(&self) -> OpenCookie {
      OpenCookie{
        id:None,
        cookie:self.name.clone(),
        category:self.category.to_cookie_category(),
        description:self.description.clone(),
      }
    }
}

#[derive(Deserialize)]
pub struct OpenTrackerCsvColumn{
    pub domain: String,
    #[serde(rename = "Ad Motivated Tracking")]
    ad_motivated_tracking: Option<u8>,
    advertising: Option<u8>,
    #[serde(rename = "Ad Fraud")]
    ad_fraud: Option<u8>,
    analytics: Option<u8>,
    #[serde(rename = "Third-Party Analytics Marketing")]
    third_party_analytics_marketing: Option<u8>,
    #[serde(rename = "Action Pixels")]
    action_pixels: Option<u8>,
    #[serde(rename = "Social Network")]
    social_network: Option<u8>,
    #[serde(rename = "Consent Management Platform")]
    consent_management_platform: Option<u8>,
    #[serde(rename = "Tag Manager")]
    tag_manager: Option<u8>,
}

impl ToCookieCategory for OpenTrackerCsvColumn {
    fn to_cookie_category(&self) -> CookieCategory {
        if self.consent_management_platform.is_some(){
            return CookieCategory::Necessary;
        }
        if self.analytics.is_some(){
            return CookieCategory::Statistics;
        }
        if self.third_party_analytics_marketing.is_some(){
            return CookieCategory::Statistics;
        }
        if self.ad_fraud.is_some(){
            return CookieCategory::Marketing;
        }
        if self.ad_motivated_tracking.is_some(){
            return CookieCategory::Marketing;
        }
        if self.advertising.is_some(){
            return CookieCategory::Marketing;
        }
        if self.social_network.is_some(){
            return CookieCategory::Marketing;
        }
        if self.tag_manager.is_some(){
            return CookieCategory::Statistics;
        }
        if self.action_pixels.is_some(){
            return CookieCategory::Marketing;
        }
        CookieCategory::Unclassified
    }
}

impl OpenTrackerCsvColumn {
    pub fn to_tracker(&self) -> OpenTracker {
       OpenTracker{
         id:None,
         domain:self.domain.clone(),
         category:self.to_cookie_category(),
       }
    }
}