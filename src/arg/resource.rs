use super::Any;
use std::any::TypeId;

pub trait ResourceConfig {
    fn get_type(&self) -> TypeId;
    fn to_any(&self) -> &dyn Any;
}

pub trait Res: Any + 'static {}

pub struct Resource<T: Res> {
    data: T,
}

impl<T: Res> Resource<T> {
    pub fn new(data: T) -> Resource<T> {
        Resource { data }
    }
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T: Res> ResourceConfig for Resource<T> {
    fn get_type(&self) -> TypeId {
        self.type_id()
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl<T: Res> Resource<T> {}
