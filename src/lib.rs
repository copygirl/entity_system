use std::any::Any;

pub mod storage;


#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Entity(pub i32);

pub trait Component: Any { }
impl<T: Any> Component for T { }
