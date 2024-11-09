use std::any::{Any, TypeId};
use system::*;

mod system;

trait ResourceTrait {
    fn get_type(&self) -> TypeId;
    fn to_any(&self) -> &dyn Any;
}

trait QueryTrait {
    fn get_type(&self) -> TypeId;
    fn to_any(&self) -> &dyn Any;
}

struct Query<T: Component> {
    components: Vec<T>,
}

struct Resource<T: Res> {
    data: T,
}

impl<T: Component> Arg for Query<T> {}
impl<T: Res> Arg for Resource<T> {}

impl<T: Component> QueryTrait for Query<T> {
    fn get_type(&self) -> TypeId {
        self.type_id()
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}
impl<T: Res> ResourceTrait for Resource<T> {
    fn get_type(&self) -> TypeId {
        self.type_id()
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

trait Component: Any + 'static {}

trait Res: Any + 'static {}

impl<T: Component> Query<T> {}
impl<T: Res> Resource<T> {}

struct App {
    systems: Vec<Box<dyn System>>,
    querys: Vec<Box<dyn QueryTrait>>,
    resources: Vec<Box<dyn ResourceTrait>>,
}

impl App {
    fn new() -> Self {
        App {
            systems: Vec::new(),
            querys: Vec::new(),
            resources: Vec::new(),
        }
    }

    fn add_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.querys.push(Box::new(Query {
            components: vec![component],
        }));
        self
    }

    fn add_resource<T: Res>(&mut self, resource: Resource<T>) -> &mut Self {
        self.resources.push(Box::new(resource));
        self
    }

    fn add_system(&mut self, system: impl System + 'static) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    fn call(&self) {
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

fn system() {
    println!("no arg system")
}

fn print_query(query: &Query<Player>) {
    for component in query.components.iter() {
        println!("{:?}", component);
    }
}

fn print_resource(resource: &Resource<Data>) {
    println!("{:?}", resource.data);
}

fn to_arg_system(query: &Query<Player>, res: &Resource<Data>) {
    println!("{:?}", query.components,);
    println!("{:?}", res.data)
}

#[derive(Debug)]
struct Player {
    name: String,
    hp: u32,
}

#[derive(Debug)]
struct Data {
    value: u32,
}

fn main() {
    App::new()
        .add_component(Player {
            name: String::from("Player"),
            hp: 100,
        })
        .add_resource(Resource {
            data: Data { value: 100 },
        })
        .add_system(Arg0System::from(system))
        .add_system(Arg1System::from(print_query))
        .add_system(Arg1System::from(print_resource))
        .add_system(Arg2System::from(to_arg_system))
        .call();
}

impl Component for Player {}
impl Res for Data {}
