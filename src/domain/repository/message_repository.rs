use async_trait::async_trait;
use eyre::Result;

use crate::domain::entity::Message;

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn fetch_by_id(&self, user_id: &str) -> Result<Vec<Message>>;
}
