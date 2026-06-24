use std::collections::HashMap;

use async_trait::async_trait;
use eyre::Result;

use crate::domain::{entity::Message, repository::MessageRepository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryMessageRepository {
    messages: HashMap<String, Message>,
}

impl InMemoryMessageRepository {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }
}

#[async_trait]
impl MessageRepository for InMemoryMessageRepository {
    async fn fetch_by_id(&self, id: &str) -> Result<Vec<Message>> {
        Ok(self
            .messages
            .get(id)
            .cloned()
            .into_iter()
            .collect::<Vec<_>>())
    }
}
