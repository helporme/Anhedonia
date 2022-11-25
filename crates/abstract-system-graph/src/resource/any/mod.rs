pub mod storage;
pub mod links;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

type AnyRwLock = RwLock<Box<dyn Any>>;
type AnyRwReadGuard<'a> = RwLockReadGuard<'a, Box<dyn Any>>;
type AnyRwWriteGuard<'a> = RwLockWriteGuard<'a, Box<dyn Any>>;
type AnyMap = HashMap<TypeId, AnyRwLock>;

