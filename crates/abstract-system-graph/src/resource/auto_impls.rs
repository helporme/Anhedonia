use std::mem;
// todo: doc
use macros::impl_with_idents;

use crate::dependency::DependencyWriter;
use crate::resource::{Link, Linker, FiniteLinker};

macro_rules! impl_link_tuple {
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<$($input: Link),*> Link for ($($input),*,) {
            #[inline]
            fn write_deps(writer: &mut DependencyWriter) {
                $($input::write_deps(writer));*;
            }
        }
    };

    () => {
        impl Link for () {
            #[inline]
            fn write_deps(_: &mut DependencyWriter) { }
        }
    }
}

macro_rules! impl_finite_linker {
    ($($L:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<'_fn, Compound: '_fn, $($L),*,> FiniteLinker<($($L),*,)> for Compound
            where $($L: Link),*, $(Compound: Linker<'_fn, $L>),* {

            fn link(&self) -> Option<($($L),*,)> {
                // todo: safety doc
                let _self: &'_fn Self = unsafe {mem::transmute(self) };

                if let ($(Some($L)),*,) = ($(Linker::<$L>::link(_self)),*,) {
                    Some(($($L),*,))
                } else {
                    None
                }
            }

            fn can_be_linked(&self) -> bool {
                $(Linker::<$L>::can_be_linked(self))&&*
            }
        }
    };
}

impl_with_idents!(impl_link_tuple, 0, 16, L);
impl_with_idents!(impl_finite_linker, 1, 16, L);
