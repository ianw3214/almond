use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};

// Direction ------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

// KeyboardControlled ------------------------------------------
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

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

// Velocity ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction
}

// Sprite ------------------------------------------
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect
}

// MovementAnimation ------------------------------------------
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>
}