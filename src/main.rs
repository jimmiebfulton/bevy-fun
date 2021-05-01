use crate::systems::hello::HelloPlugin;
use bevy::prelude::*;

mod components;
mod systems;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(systems::init::init.system())
        .run();
}
