use async_trait::async_trait;
use eyre::Result;

use crate::domain::entity::User;

#[async_trait]
pub trait UserRepository: std::any::Any + Send + Sync {
    async fn fetch_by_id(&self, user_id: &str) -> Result<Option<User>>;
}
