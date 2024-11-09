use super::Any;
use std::any::TypeId;

pub trait QueryConfig {
    fn get_type(&self) -> TypeId;
    fn to_any(&self) -> &dyn Any;
}

pub struct Query<T: Component> {
    components: Vec<T>,
}

impl<T: Component> Query<T> {
    pub fn new(components: Vec<T>) -> Query<T> {
        Query { components }
    }
    pub fn components(&self) -> &Vec<T> {
        &self.components
    }
}

impl<T: Component> QueryConfig for Query<T> {
    fn get_type(&self) -> TypeId {
        self.type_id()
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

pub trait Component: Any + 'static {}

impl<T: Component> Query<T> {}
