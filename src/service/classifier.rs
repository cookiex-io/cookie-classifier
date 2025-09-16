use crate::{dto::cookie_dto::{CookiePromptResponse, CookieRequest, CookieResponse}};
use rig::{agent::Agent, completion::Prompt, providers::openai::CompletionModel};
use anyhow::Error;

pub const COOKIE_CLASSIFICATION_PROMPT: &str = r#"
You are given a cookie with the following attributes:
- Name: {name}
- Provider: {provider}

Classify this cookie into exactly one of the following categories:
1. Necessary: Required for basic site operation, authentication, or security.
2. Marketing: Used for advertising, tracking across sites, personalization of ads.
3. Statistics: Used for analytics, measuring website usage, or performance metrics.
4. Preference: Stores user settings such as language, theme, or region.
5. Unclassified: Purpose unclear or insufficient information.

Return only the category adn description in json text like this {"category":"Necessary","description":"description text here"}.
"#;

pub trait LLMClassification {
    async fn classify_cookie(&self,req:CookieRequest) -> Result<(String,CookieResponse),Error>;
}

impl LLMClassification for Agent<CompletionModel> {
    async fn classify_cookie(&self,req:CookieRequest) -> Result<(String,CookieResponse),Error> {
      let prompt = req.to_prompt();
      let response = self.prompt(prompt).await?;
      println!("{response}");
      let prompt_responnse:CookiePromptResponse = serde_json::from_str(&response)?;
      let cookie = CookieResponse{
            provider: req.provider,
            category:prompt_responnse.category,
            description:prompt_responnse.description.into(),
      };
      Ok((req.name,cookie))
    }
}