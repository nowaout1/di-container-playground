use std::collections::HashMap;

use async_trait::async_trait;
use eyre::Result;

use crate::domain::{entity::User, repository::UserRepository};

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
