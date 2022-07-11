use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};

// WorldPosition ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct WorldPosition(pub Point);

// GridPosition ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GridPosition {
    pub x : i32,
    pub y : i32
}

// Sprite ------------------------------------------
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect
}

// Animation ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Animation {
    pub current_frame: usize,
    pub current_anim: usize,
    pub animations: Vec<(usize, usize)>
}

// Brain ------------------------------------------
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Brain;

// Health ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Health {
    pub health : i32,
    pub max_health : i32
}

// Selectable ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Selectable {
    pub width : i32,
    pub height : i32,
    pub selected : bool
}

// Turn ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Turn {
    pub current : bool
}