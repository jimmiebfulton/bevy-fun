use bevy::prelude::*;

use crate::components::{Animal, Creature, Name, Person};

pub fn init(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn()
        .insert(Creature)
        .insert(Person)
        .insert(Name(String::from("Jimmie")));
    commands
        .spawn()
        .insert(Creature)
        .insert(Person)
        .insert(Name(String::from("Shirley")));
    commands
        .spawn()
        .insert(Creature)
        .insert(Animal)
        .insert(Name(String::from("Bailey")));
}
