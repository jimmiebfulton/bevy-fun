use bevy::prelude::*;

pub struct HelloPlugin;

use crate::components::Name;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(say_hello.system());
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
        // app.add_system(say_hello.system());
        app.add_system(greet.system());
    }
}

pub fn say_hello() {
    print!("Hello, Cruel World!");
}

fn greet(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(Entity, &Name)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (entity, name) in query.iter() {
            println!("({:?}) Hello, {}!", entity, name.0);
        }
    }
}

struct GreetTimer(Timer);
