use macros::impl_with_idents;

use crate::resource::Resource;
use crate::system::System;


/// Macro that implements system for function with a given parameter generics.
///
/// For example:
/// ```rust
/// impl_system_for_func!(In0, In1);
/// ```
macro_rules! impl_system_for_func {
    // Handle more then zero generics
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier ('In')
        impl<Func, $($input),*> System<($($input),*,)> for Func
            where Func: Fn($($input),*), $($input: Resource),* {

            #[inline]
            fn run(&mut self, input: ($($input),*,)) {
                let ($($input),*,) = input;
                self($($input),*)
            }
        }
    };

    // Handle zero generics
    () => {
        impl<Func> System<()> for Func
            where Func: Fn() {

            #[inline]
            fn run(&mut self, _: ()) {
                self()
            }
        }
    }
}

// Invoke `impl_system_for_func` for all parameter sets with a length of 0 to 16.
impl_with_idents!(impl_system_for_func, In, 0, 16);
