use std::sync::Arc;

use di_container_playground::{Container, Injectable, Resolver};

use crate::{
    application::service::messenger::{Dialog, Messenger},
    infrastructure::data::local::{InMemoryMessageRepository, InMemoryUserRepository},
};

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut container = Container::new();

    container
        .register(InMemoryUserRepository::new())
        .register(InMemoryMessageRepository::new());

    let messenger = container.resolve::<Messenger>()?;
    let _dialog: Dialog = messenger.load_dialog("user:123").await?;

    Ok(())
}

impl Injectable for Messenger {
    type Dependencies = (Arc<InMemoryUserRepository>, Arc<InMemoryMessageRepository>);

    fn inject((users, messages): Self::Dependencies) -> Self {
        Self { users, messages }
    }
}
