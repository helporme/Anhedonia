//! [`System`] implementation for all functions whose parameters are [`Link`].

use std::marker::PhantomData;
use macros::impl_with_idents;

use crate::linking::Link;
use crate::system::System;

pub struct FunctionSystem<Func: SystemFn<In, Out>, In: Link, Out> {
    func: Func,
    _marker: PhantomData<(In, Out)>
}

impl<Func: SystemFn<In, Out>, In: Link, Out> System for FunctionSystem<Func, In, Out> {
    type Input = In;
    type Output = Out;

    fn run(&self, input: Self::Input) -> Self::Output {
        self.func.call(input)
    }
}

impl<Func: SystemFn<In, Out>, In: Link, Out> From<Func> for FunctionSystem<Func, In, Out> {
    fn from(func: Func) -> Self {
        Self { func, _marker: PhantomData }
    }
}

/// All suitable functions for [`FunctionSystem`] automatically implement this trait.
pub trait SystemFn<Input: Link, Output> {
    fn call(&self, input: Input) -> Output;
}

/// Macro that implements [`SystemFn`] for [`Fn`] trait.
///
/// # Examples
/// ```
/// // impl_system_fn(L0, L1, L2) will generate the following code:
///
/// #[allow(non_snake_case)]
/// impl<Func, Output, L0, L1, L2> SystemFn<(L0, L1, L2,), Output> for Func
///     where Func: Fn(L0, L1, L2) -> Output, L0: Link, L1: Link, L2: Link {
///
///     #[inline(always)]
///     fn call(&self, input: (L0, L1, L2,)) -> Output {
///         // Generics are shadowed and used as variables
///         let (L0, L1, L2) = input;
///
///         // Invoke `Func`
///         self(L0, L1, L2)
///     }
/// }
/// ```
macro_rules! impl_system_fn {
    // for more than zero generics
    ($($input:ident),+) => {
        #[allow(non_snake_case)] // Triggers on generic identifier
        impl<Func, Output, $($input),*> SystemFn<($($input),*,), Output> for Func
            where Func: Fn($($input),*) -> Output, $($input: Link),* {

            #[inline(always)]
            fn call(&self, input: ($($input),*,)) -> Output {
                let ($($input),*,) = input;
                self($($input),*)
            }
        }
    };

    // for zero generics
    () => {
        impl<Output, Func> SystemFn<(), Output> for Func
            where Func: Fn() -> Output {

            #[inline(always)]
            fn call(&self, _: ()) -> Output {
                self()
            }
        }
    }
}

// Invoke macro for all parameter sets with a length of 0 to 16.
impl_with_idents!(impl_system_fn, 0, 16, L);
