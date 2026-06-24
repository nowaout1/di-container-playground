use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

// TODO: typed errors (thiserror)
use eyre::{ContextCompat, Result};

#[derive(Debug, Clone)]
pub struct Container {
    deps: HashMap<TypeId, Arc<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, item: T) -> &mut Self
    where
        T: Any,
    {
        let id = TypeId::of::<T>();
        self.deps.insert(id, Arc::new(item));
        self
    }

    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();

        let Some(item) = self.deps.get(&id) else {
            return None;
        };

        item.downcast_ref().cloned()
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
        let d1 = self
            .get::<D1>()
            .with_context(|| format!("dependency {:?} not found", std::any::type_name::<D1>()))?;

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
        let d1 = self
            .get::<D1>()
            .with_context(|| format!("dependency {:?} not found", std::any::type_name::<D1>()))?;

        let d2 = self
            .get::<D2>()
            .with_context(|| format!("dependency {:?} not found", std::any::type_name::<D2>()))?;

        Ok(I::inject((d1, d2)))
    }
}

pub trait Resolver<D> {
    fn resolve<I>(&self) -> Result<I>
    where
        I: Injectable<Dependencies = D>;
}

pub trait Injectable: Send + Sync {
    type Dependencies;

    fn inject(deps: Self::Dependencies) -> Self;
}
