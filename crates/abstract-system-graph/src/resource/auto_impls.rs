// todo: doc
use macros::impl_with_idents;

use crate::dependency::DependencyWriter;
use crate::resource::{Link, Linker, LinkerCompound};

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

macro_rules! impl_linker_compound {
    ($($L:ident),+) => {
        impl<Compound, $($L),*,> LinkerCompound<($($L),*,)> for Compound
            where $($L: Link),*, $(Compound: Linker<$L>),* {

            fn link(&self) -> Option<($($L),*,)> {
                if let ($(Some($L)),*,) = ($(Linker::<$L>::link(self)),*,) {
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

    () => {
        impl<Compound: Linker<()>> LinkerCompound<()> for Compound {
            fn link(&self) -> Option<()> {
                Some(())
            }

            fn can_be_linked(&self) -> bool {
                true
            }
        }
    }
}

impl_with_idents!(impl_link_tuple, 0, 16, L);
impl_with_idents!(impl_linker_compound, 0, 16, L);
