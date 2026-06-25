use std::sync::Arc;

use di::Injectable;
use eyre::Result;

use crate::domain::{
    entity::{Message, User},
    repository::{MessageRepository, UserRepository},
};

#[derive(Clone)]
pub struct Messenger {
    pub users: Arc<dyn UserRepository>,
    pub messages: Arc<dyn MessageRepository>,
}

impl<U, M> Injectable<(Arc<U>, Arc<M>)> for Messenger
where
    U: UserRepository,
    M: MessageRepository,
{
    fn inject((users, messages): (Arc<U>, Arc<M>)) -> Self {
        Self { users, messages }
    }
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
