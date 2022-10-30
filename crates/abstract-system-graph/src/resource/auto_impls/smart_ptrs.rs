// todo: doc

use std::cell::{Cell, RefCell};
use std::sync::{Mutex, RwLock};

use crate::resource::Link;
use crate::dependency::{Dependency, DependencyWriter};

macro_rules! impl_channel_with_deps {
    ($ptr:ty, $dep:ident, $dep_ref:ident, $dep_mut:ident) => {
        impl_channel_with_deps!($ptr, $dep);
        impl_channel_with_deps!(&$ptr, $dep_ref);
        impl_channel_with_deps!(&mut $ptr, $dep_mut);
    };

    ($ptr:ty, $dep:ident) => {
        impl<T: 'static> Link for $ptr {
            fn write_deps(writer: &mut DependencyWriter) {
                writer.write(Dependency::$dep::<T>())
            }
        }
    }
}

impl_channel_with_deps!(Box<T>, write_of, read_of, write_of);
impl_channel_with_deps!(Cell<T>, write_of, write_of, write_of);
impl_channel_with_deps!(RefCell<T>, write_of, write_of, write_of);
impl_channel_with_deps!(RwLock<T>, write_of, write_of, write_of);
impl_channel_with_deps!(Mutex<T>, write_of, write_of, write_of);
