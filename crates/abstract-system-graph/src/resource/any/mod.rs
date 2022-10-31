// todo: doc

pub mod links;
pub mod storage;
pub mod channels;

pub use links::{Lock, Ref, Mut};
pub use storage::AnyStorage;
pub use channels::LockChannel;

pub trait AnyResource: 'static + Default { }

impl<R: 'static + Default> AnyResource for R { }