use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use hashbrown::HashMap;
// TODO: typed errors (thiserror)
use eyre::{ContextCompat, Result};

pub fn inject<I, Deps>(container: &Container) -> Result<I>
where
    I: Injectable<Deps>,
    Container: Resolver<I, Deps>,
{
    container.resolve()
}

#[derive(Debug, Clone)]
pub struct Container {
    deps: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, item: T) -> &mut Self
    where
        T: Any + Send + Sync,
    {
        let id = TypeId::of::<T>();
        self.deps.insert(id, Arc::new(item));
        self
    }

    pub fn get<T>(&self) -> Option<Arc<dyn Any + Send + Sync>>
    where
        T: 'static,
    {
        self.deps.get(&TypeId::of::<T>()).cloned()
    }
}

impl<I, I1> Resolver<I, (Arc<I1>,)> for Container
where
    I: Injectable<(Arc<I1>,)>,
    I1: Send + Sync + 'static,
{
    fn resolve(&self) -> Result<I> {
        let a1 = self.get::<I1>().with_context(|| {
            format!(
                "failed to resolve dependency {:?}",
                std::any::type_name::<I1>()
            )
        })?;

        let i1 = Arc::downcast::<I1>(a1).unwrap();

        Ok(I::inject((i1,)))
    }
}

impl<I, I1, I2> Resolver<I, (Arc<I1>, Arc<I2>)> for Container
where
    I: Injectable<(Arc<I1>, Arc<I2>)>,
    I1: Send + Sync + 'static,
    I2: Send + Sync + 'static,
{
    fn resolve(&self) -> Result<I> {
        let a1 = self.get::<I1>().with_context(|| {
            format!(
                "failed to resolve dependency {:?}",
                std::any::type_name::<I1>()
            )
        })?;
        let a2 = self.get::<I2>().with_context(|| {
            format!(
                "failed to resolve dependency {:?}",
                std::any::type_name::<I2>()
            )
        })?;

        let i1 = Arc::downcast::<I1>(a1).unwrap();
        let i2 = Arc::downcast::<I2>(a2).unwrap();

        Ok(I::inject((i1, i2)))
    }
}

pub trait Resolver<I, Deps>
where
    I: Injectable<Deps>,
{
    fn resolve(&self) -> Result<I>;
}

pub trait Injectable<Deps>: Send + Sync {
    fn inject(deps: Deps) -> Self;
}
