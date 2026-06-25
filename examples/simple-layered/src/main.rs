use std::sync::Arc;

use crate::{
    application::service::messenger::{Dialog, Messenger},
    infrastructure::data::{
        local::{InMemoryMessageRepository, InMemoryUserRepository},
        remote::{PostgresMessageRepository, PostgresUserRepository},
    },
};

mod application;
mod domain;
mod infrastructure;

type MixedDeps = (Arc<InMemoryUserRepository>, Arc<PostgresMessageRepository>);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut container = di::Container::new();

    container
        .register(InMemoryUserRepository::new())
        .register(InMemoryMessageRepository::new());

    container
        .register(PostgresUserRepository::new())
        .register(PostgresMessageRepository::new());

    let messenger = di::inject::<Messenger, MixedDeps>(&container)?;
    let _dialog: Dialog = messenger.load_dialog("user:123").await?;

    Ok(())
}
