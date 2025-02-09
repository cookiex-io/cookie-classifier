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
    #[serde(rename = "ID")]
    id:Option<String>,
    #[serde(rename = "Platform")]
    platform:String,
    #[serde(rename = "Category")]
    category:OpenCookieCategory,
    #[serde(rename = "Cookie / Data Key name")]
    pub name:String,
    #[serde(rename = "Domain")]
    domain:String,
    #[serde(rename = "Description")]
    description:String,
    #[serde(rename = "Retention period")]
    retention_period:Option<String>,
    #[serde(rename = "Data Controller")]
    data_controller:Option<String>,
    #[serde(rename = "User Privacy & GDPR Rights Portals")]
    user_privacy_and_gdpre_rights_portals:String,
    #[serde(rename = "Wildcard match")]
    wildcard_match:Option<String>,
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
    #[serde(rename = "Audience Measurement")]
    audience_measurement: Option<u8>,
    #[serde(rename = "Federated Login")]
    federated_login: Option<u8>,
    sso: Option<u8>,
    #[serde(rename = "Third-Party Analytics Marketing")]
    third_party_analytics_marketing: Option<u8>,
    #[serde(rename = "Social - Comment")]
    social_comment: Option<u8>,
    #[serde(rename = "Social - Share")]
    social_share: Option<u8>,
    #[serde(rename = "Online Payment")]
    online_payment: Option<u8>,
    #[serde(rename = "Action Pixels")]
    action_pixels: Option<u8>,
    #[serde(rename = "Unknown High Risk Behavior")]
    unknown_high_risk_behavior: Option<u8>,
    #[serde(rename = "Obscure Ownership")]
    obscure_ownership: Option<u8>,
    cdn: Option<u8>,
    badge: Option<u8>,
    #[serde(rename = "Embedded Content")]
    embedded_content: Option<u8>,
    #[serde(rename = "Session Replay")]
    session_replay: Option<u8>,
    #[serde(rename = "Social Network")]
    social_network: Option<u8>,
    non_tracking: Option<u8>,
    malware: Option<u8>,
    #[serde(rename = "Fraud Prevention")]
    fraud_prevention: Option<u8>,
    #[serde(rename = "Consent Management Platform")]
    consent_management_platform: Option<u8>,
    #[serde(rename = "Tag Manager")]
    tag_manager: Option<u8>,
    #[serde(rename = "Support Chat Widget")]
    support_chat_widget: Option<u8>,
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