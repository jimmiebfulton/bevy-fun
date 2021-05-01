use crate::components::{Animal, Creature, Name, Person, Position};
use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    commands.spawn().insert(Position::default());
    commands.spawn().insert(Position { x: 5.1, y: 210.5 });

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
