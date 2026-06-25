use async_trait::async_trait;
use eyre::Result;

use crate::domain::{entity::User, repository::UserRepository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostgresUserRepository {}

impl PostgresUserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn fetch_by_id(&self, id: &str) -> Result<Option<User>> {
        unimplemented!()
    }
}
