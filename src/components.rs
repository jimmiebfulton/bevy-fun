#[derive(Debug)]
pub struct Name(pub String);

pub struct Creature;

pub struct Person;

pub struct Animal;

#[derive(Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
