//! Implementation of [`Link`] for all tuples whose elements are also [`Link`].

use macros::impl_with_idents;

use crate::dependency::DependencyWriter;
use crate::resource::*;

/// The macro that implements the [`Link`] trait for a tuple whose elements are links.
macro_rules! impl_link_tuple {
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<$($input: Link),*> Link for ($($input),*,) {
            type Resource = ($($input::Resource),*,);

            #[inline]
            fn write_dependencies(writer: &mut DependencyWriter) {
                $($input::write_dependencies(writer));*;
            }
        }
    };

    () => {
        impl Link for () {
            type Resource = ();

            #[inline]
            fn write_dependencies(_: &mut DependencyWriter) { }
        }
    }
}

/// The macro that implements the [`LinkSupport`] trait for a tuple whose elements are links.
macro_rules! impl_link_tuple_support {
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<'a, $($input),*, Linker> LinkSupport<'a, ($($input),*,)> for Linker
            where $($input: Link),*,
                  Linker: ResourceLinker<'a> $(+ LinkSupport<'a, $input>)* {
            fn acquire_link(&self) -> Option<($($input),*,)> {
                if let ($(Some($input)),*,) =
                    ($(LinkSupport::<'a, $input>::acquire_link(self)),*,) {

                    Some(($($input),*,))
                } else {
                    None
                }
            }

            fn has_link(&self) -> bool {
                true $(&& LinkSupport::<'a, $input>::has_link(self))*
            }
        }
    };

    () => {
        impl<'a, Linker: ResourceLinker<'a>> LinkSupport<'a, ()> for Linker {
            fn acquire_link(&self) -> Option<()> {
                Some(())
            }

            fn has_link(&self) -> bool {
                true
            }
        }
    }
}

// Invoke macros for all generic sets with a length of 0 to 16.
impl_with_idents!(impl_link_tuple, L, 0, 16);
impl_with_idents!(impl_link_tuple_support, L, 0, 16);
