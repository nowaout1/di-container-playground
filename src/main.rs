use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use eyre::{ContextCompat, Result};

mod template;

use template::*;

// TODO: typed errors (thiserror)

#[tokio::main]
async fn main() -> Result<()> {
    let mut container = Container::new();

    container
        .set(InMemoryUserRepository::new())
        .set(InMemoryMessageRepository::new());

    let messenger = container.resolve::<Messenger>()?;
    let _dialog: Dialog = messenger.load_dialog("123").await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct Container {
    deps: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, item: T) -> &mut Self
    where
        T: Any + Send + Sync,
    {
        let id = TypeId::of::<T>();
        self.deps.insert(id, Arc::new(item));
        self
    }

    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        let id = TypeId::of::<T>();

        let Some(item) = self.deps.get(&id).cloned() else {
            return None;
        };

        item.downcast::<T>().ok()
    }
}

impl Resolver<()> for Container {
    fn resolve<I>(&self) -> Result<I>
    where
        I: Injectable<Dependencies = ()>,
    {
        Ok(I::inject(()))
    }
}

impl<D1> Resolver<(Arc<D1>,)> for Container
where
    D1: Send + Sync + 'static,
{
    fn resolve<I>(&self) -> Result<I>
    where
        I: Injectable<Dependencies = (Arc<D1>,)>,
    {
        let d1 =
            Arc::clone(&self.get::<D1>().with_context(|| {
                format!("dependency {:?} not found", std::any::type_name::<D1>())
            })?);

        Ok(I::inject((d1,)))
    }
}

impl<D1, D2> Resolver<(Arc<D1>, Arc<D2>)> for Container
where
    D1: Send + Sync + 'static,
    D2: Send + Sync + 'static,
{
    fn resolve<I>(&self) -> Result<I>
    where
        I: Injectable<Dependencies = (Arc<D1>, Arc<D2>)>,
    {
        let d1 =
            Arc::clone(&self.get::<D1>().with_context(|| {
                format!("dependency {:?} not found", std::any::type_name::<D1>())
            })?);

        let d2 =
            Arc::clone(&self.get::<D2>().with_context(|| {
                format!("dependency {:?} not found", std::any::type_name::<D2>())
            })?);

        Ok(I::inject((d1, d2)))
    }
}

trait Resolver<D> {
    fn resolve<I>(&self) -> Result<I>
    where
        I: Injectable<Dependencies = D>;
}

impl Injectable for Messenger {
    type Dependencies = (Arc<InMemoryUserRepository>, Arc<InMemoryMessageRepository>);

    fn inject((users, messages): Self::Dependencies) -> Self {
        Self { users, messages }
    }
}

pub trait Injectable: Send + Sync {
    type Dependencies;

    fn inject(deps: Self::Dependencies) -> Self;
}
