use async_trait::async_trait;
use eyre::Result;

use crate::domain::{entity::Message, repository::MessageRepository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostgresMessageRepository {}

impl PostgresMessageRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MessageRepository for PostgresMessageRepository {
    async fn fetch_by_id(&self, id: &str) -> Result<Vec<Message>> {
        unimplemented!()
    }
}
