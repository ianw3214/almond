use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Renderable {
    pub i: usize
}

#[derive(Component)]
pub struct Animatable {
    // image data
    pub width: i32,
    pub height: i32,
    // game data
    pub frame: i32
}