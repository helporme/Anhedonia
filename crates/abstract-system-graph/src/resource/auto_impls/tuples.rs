// todo: doc
use macros::impl_with_idents;

use crate::dependency::DependencyWriter;
use crate::resource::{Link, Channel, ChannelEstablisher};

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

macro_rules! impl_channel_tuple {
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<'a, $($input: Link),*, Compound> Channel<'a, ($($input),*,)> for Compound
            where $(Compound: Channel<'a, $input>),* {

            #[inline]
            fn obtain(&'a self) -> Option<($($input),*,)> {
                if let ($(Some($input)),*,) = ($(Channel::<'a, $input>::obtain(self)),*,) {
                    Some(($($input),*,))
                } else {
                    None
                }
            }

            #[inline]
            fn is_alive(&self) -> bool {
                $(Channel::<'a, $input>::is_alive(self))&&*
            }
        }
    };

    () => {
        impl<'a, T> Channel<'a, ()> for T {
            #[inline]
            fn obtain(&self) -> Option<()> {
                Some(())
            }

            #[inline]
            fn is_alive(&self) -> bool {
                true
            }
        }
    }
}

macro_rules! impl_channel_establisher_tuple {
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<'a, $($input),*, Compound> ChannelEstablisher<'a, ($($input),*,)> for Compound
            where $(Compound: ChannelEstablisher<'a, $input>),* {

            #[inline]
            fn configure(&mut self) {
                $(ChannelEstablisher::<'a, $input>::configure(self));*
            }

            #[inline]
            fn establish(&'a self) -> Option<($($input),*,)> {
                if let ($(Some($input)),*,) =
                    ($(ChannelEstablisher::<'a, $input>::establish(self)),*,) {

                    Some(($($input),*,))
                } else {
                    None
                }
            }
        }
    };

    () => {
        impl<T> ChannelEstablisher<'_, ()> for T {
            fn configure(&mut self) { }

            fn establish(&self) -> Option<()> {
                Some(())
            }
        }
    }
}

impl_with_idents!(impl_link_tuple, L, 0, 16);
impl_with_idents!(impl_channel_tuple, L, 0, 16);
impl_with_idents!(impl_channel_establisher_tuple, C, 0, 16);
