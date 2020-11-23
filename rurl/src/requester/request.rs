use async_trait::async_trait;

#[async_trait]
pub trait Request {
    async fn request(&self, url: String) -> Result<String, Box<dyn std::error::Error>>;
}
