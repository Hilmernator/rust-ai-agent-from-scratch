use super::types::{ModelRequest, ModelResponse};
use anyhow::Result;

pub trait ModelAdapter: Send + Sync {
    async fn send(&self, request: ModelRequest) -> Result<ModelResponse>;
}
