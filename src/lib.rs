use std::{
    any::{Any, TypeId, type_name},
    sync::Arc,
};

use hashbrown::HashMap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    #[error("dependency {0} not found")]
    NotFound(String),

    #[error("incompatible type provided {0}")]
    Incompatible(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn resolve<I, Deps>(container: &Container) -> Result<I>
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

    pub fn get<T>(&self) -> Result<Arc<T>>
    where
        T: Send + Sync + 'static,
    {
        let Some(item) = self.deps.get(&TypeId::of::<T>()).cloned() else {
            return Err(Error::NotFound(type_name::<T>().to_string()));
        };

        Arc::downcast::<T>(item).map_err(|_| Error::Incompatible(type_name::<T>().to_string()))
    }
}

macro_rules! impl_resolver {
    ($($T:ident),+) => {
        impl<I, $($T),+> Resolver<I, ($(Arc<$T>,)+)> for Container
        where
            I: Injectable<($(Arc<$T>,)+)>,
            $($T: Send + Sync + 'static,)+
        {
            fn resolve(&self) -> Result<I> {
                let deps = (
                    $(
                        self.get::<$T>()?,
                    )+
                );

                Ok(I::inject(deps))
            }
        }
    };
}

impl_resolver!(I1);
impl_resolver!(I1, I2);
impl_resolver!(I1, I2, I3);
impl_resolver!(I1, I2, I3, I4);
impl_resolver!(I1, I2, I3, I4, I5);
impl_resolver!(I1, I2, I3, I4, I5, I6);
impl_resolver!(I1, I2, I3, I4, I5, I6, I7);
impl_resolver!(I1, I2, I3, I4, I5, I6, I7, I8);
impl_resolver!(I1, I2, I3, I4, I5, I6, I7, I8, I9);
impl_resolver!(I1, I2, I3, I4, I5, I6, I7, I8, I9, I10);
impl_resolver!(I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11);
impl_resolver!(I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12);

pub trait Resolver<I, Deps>
where
    I: Injectable<Deps>,
{
    fn resolve(&self) -> Result<I>;
}

pub trait Injectable<Deps>: Send + Sync {
    fn inject(deps: Deps) -> Self;
}
