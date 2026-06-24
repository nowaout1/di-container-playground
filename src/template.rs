use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use eyre::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryUserRepository {
    users: HashMap<String, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn fetch_by_id(&self, id: &str) -> Result<Option<User>> {
        Ok(self.users.get(id).cloned())
    }
}

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

#[derive(Clone)]
pub struct Messenger {
    pub users: Arc<dyn UserRepository>,
    pub messages: Arc<dyn MessageRepository>,
}

impl Messenger {
    pub async fn load_dialog(&self, user_id: &str) -> Result<Dialog> {
        let (maybe_user, messages) = tokio::try_join!(
            self.users.fetch_by_id(user_id),
            self.messages.fetch_by_id(user_id)
        )?;

        let Some(user) = maybe_user else {
            return Err(eyre::eyre!("user not found"));
        };

        Ok(Dialog { user, messages })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dialog {
    user: User,
    messages: Vec<Message>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn fetch_by_id(&self, user_id: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn fetch_by_id(&self, user_id: &str) -> Result<Vec<Message>>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message {
    pub id: String,
}
