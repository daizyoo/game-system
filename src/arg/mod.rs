mod query;
mod resource;

pub use query::*;
pub use resource::*;

use std::any::Any;

pub trait Arg: Any + 'static {}

impl<T: Component> Arg for Query<T> {}
impl<T: Res> Arg for Resource<T> {}
