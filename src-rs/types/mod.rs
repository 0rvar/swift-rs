pub mod array;
pub mod data;
pub mod string;
pub mod object;

pub use array::*;
pub use data::*;
pub use string::*;
pub use object::*;

#[derive(Debug)]
pub struct Ptr<T>(pub *mut T);