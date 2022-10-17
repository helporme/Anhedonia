use std::any::TypeId;
use macros::impl_with_idents;

use crate::dependency::Dependency;


/// Trait which is implemented by all types that can act as resource in the [`SystemGraph`].
pub trait Resource: Sized {
    type Meta: ResourceMeta<Self>;
    type Extractor: ResourceExtractor<Self>;
}

pub trait ResourceMeta<R: Resource> {
    fn dependencies() -> [Dependency];
    fn id() -> ResourceId;
}

pub trait ResourceExtractor<R: Resource> {
    fn extract(storage: &impl ResourceStorage) -> R;
}

pub struct ResourceId(TypeId);

/// Trait which is implemented by all tuples of the [`Resource`].
pub trait ResourceTuple {
    fn get(storage: &impl ResourceStorage) -> Self;
}

/// Macro that implements [`ResourceTuple`] to the tuple of [`Resource`] with a given generics.
///
/// For example:
/// ```rust
/// impl_resource_tuple!(R0, R1);
/// ```
macro_rules! impl_resource_tuple {
    // Handle more then zero parameters
    ($($input:ident),+) => {
        impl<$($input),*> ResourceTuple for ($($input),*,)
            where $($input: Resource),*
        {
            #[inline]
            fn get(storage: &impl ResourceStorage) -> Self {
                ($($input::Extractor::extract(storage)),* ,)
            }
        }
    };

    // Handle zero parameters
    () => {
        impl ResourceTuple for () {
            #[inline]
            fn get(_: &impl ResourceStorage) -> Self {
                ()
            }
        }
    }
}

// Invoke `impl_resource_tuple` for all generic sets with a length of 0 to 16.
impl_with_idents!(impl_resource_tuple, R, 0, 16);

/// Storage
pub trait ResourceStorage {
    fn retrieve<R: Resource>(&self) -> R;
    fn store<R: Resource>(&self, resource: R);
}
