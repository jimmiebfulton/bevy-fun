use crate::systems::hello::HelloPlugin;
use crate::systems::inputs::InputsPlugin;
use bevy::prelude::*;

mod components;
mod systems;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.09, 0.09, 0.09)))
        .add_plugins(DefaultPlugins)
        .add_plugin(InputsPlugin)
        .add_plugin(HelloPlugin)
        .add_startup_system(systems::init::init.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
