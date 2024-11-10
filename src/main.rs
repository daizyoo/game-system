use std::any::Any;

use arg::*;
use system::*;

mod arg;
mod system;

struct World {}

struct App {
    world: World,
    systems: Vec<Box<dyn System>>,
    querys: Vec<Box<dyn QueryConfig>>,
    resources: Vec<Box<dyn ResourceConfig>>,
}

impl App {
    fn new() -> Self {
        App {
            world: World {},
            systems: Vec::new(),
            querys: Vec::new(),
            resources: Vec::new(),
        }
    }

    fn add_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.querys.push(Box::new(Query::new(vec![component])));
        self
    }

    fn add_resource<R: Res>(&mut self, resource: R) -> &mut Self {
        self.resources.push(Box::new(Resource::new(resource)));
        self
    }

    fn add_system(&mut self, system: impl System + 'static) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    fn run(&self) {
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
    for component in query.components() {
        println!("hp: {}", component.hp);
        println!("name: {}", component.name);
    }
}

fn print_resource(resource: &Resource<Data>) {
    let data = resource.data();
    println!("value: {}", data.value);
}

fn to_arg_system(query: &Query<Player>, res: &Resource<Data>) {
    for component in query.components() {
        println!("{:?}", component);
    }
    println!("{:?}", res.data());
}

fn to_query_system(player: &Query<Player>, enemy: &Query<Enemy>) {
    for component in player.components() {
        println!("{:?}", component);
    }
    for component in enemy.components() {
        println!("{:?}", component);
    }
}

fn main() {
    App::new()
        .add_component(Player {
            name: String::from("Player"),
            hp: 100,
        })
        .add_component(Enemy {
            name: String::from("Enemy"),
            hp: 50,
        })
        .add_resource(Data { value: 100 })
        .add_system(Arg0System::from(system))
        .add_system(Arg1System::from(print_query))
        .add_system(Arg1System::from(print_resource))
        .add_system(Arg2System::from(to_arg_system))
        .add_system(Arg2System::from(to_query_system))
        .run();
}

#[allow(dead_code)]
#[derive(Debug)]
struct Enemy {
    name: String,
    hp: u32,
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

impl Component for Player {}
impl Component for Enemy {}
impl Res for Data {}
