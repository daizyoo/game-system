use std::any::Any;

use crate::arg::*;
use crate::system::*;
use crate::world::World;
pub struct App {
    world: World,
    systems: Vec<Box<dyn System>>,
    querys: Vec<Box<dyn QueryConfig>>,
    resources: Vec<Box<dyn ResourceConfig>>,
}

impl App {
    pub fn new() -> Self {
        App {
            world: World {},
            systems: Vec::new(),
            querys: Vec::new(),
            resources: Vec::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.querys.push(Box::new(Query::new(vec![component])));
        self
    }

    pub fn add_resource<R: Res>(&mut self, resource: R) -> &mut Self {
        self.resources.push(Box::new(Resource::new(resource)));
        self
    }

    pub fn add_system(&mut self, system: impl System + 'static) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run(&self) {
        for system in &self.systems {
            let mut args: Vec<&dyn Any> = vec![];

            for ty in system.arg_type() {
                if let Some(arg) = &self.querys.iter().find(|q| q.get_type() == ty) {
                    let arg = &***arg;
                    let arg = arg.to_any();
                    args.push(arg);
                }

                if let Some(arg) = &self.resources.iter().find(|q| q.get_type() == ty) {
                    let arg = &***arg;
                    let arg = arg.to_any();
                    args.push(arg);
                }
            }

            if args.len() != system.arg_count() {
                panic!("Invalid argument count");
            }

            system.call(args);
        }
    }
}
