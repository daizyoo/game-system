use app::App;
use arg::*;
use system::*;

mod app;
mod arg;
mod system;
mod world;

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
