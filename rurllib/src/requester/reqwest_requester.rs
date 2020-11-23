use super::request::Request;
use async_trait::async_trait;
use reqwest::Url;
use std::error::Error;

pub struct ReqwestRequester {}

impl Default for ReqwestRequester {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl Request for ReqwestRequester {
    async fn request(&self, url: String) -> Result<String, Box<dyn Error>> {
        let url = Url::parse(url.as_str()).unwrap();
        let body = reqwest::get(url).await?.text().await?;
        Ok(body)
    }
}
