use bevy::prelude::*;

struct Creature;

struct Person;

struct Animal;

#[derive(Debug)]
struct Name(String);

fn init(mut commands: Commands) {
    commands.spawn().insert(Position::default());
    commands.spawn().insert(Position {
        x: 5.1,
        y: 210.5,
    });

    commands.spawn().insert(Creature).insert(Person).insert(Name(String::from("Jimmie")));
    commands.spawn().insert(Creature).insert(Person).insert(Name(String::from("Shirley")));
    commands.spawn().insert(Creature).insert(Animal).insert(Name(String::from("Bailey")));
}

fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        println!("{:?}: {:#?}", entity, position);
    }
}

fn say_hello() {
    print!("Hello, World!");
}

fn greet(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(Entity, &Name)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (entity, name) in query.iter() {
            println!("({:?}) Hello, {}!", entity, name.0);
        }
    }
}

struct GreetTimer(Timer);

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
        // app.add_system(say_hello.system());
        app.add_system(greet.system());
    }
}

#[derive(Debug, Default)]
struct Position {
    pub x: f32,
    pub y: f32,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(init.system())
        // .add_system(print_position.system())
        .run();
}
