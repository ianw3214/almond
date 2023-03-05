use bevy::{prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct WorldPosition {
    pub x : f32,
    pub y : f32
}

#[derive(Component, Default)]
pub struct Movement {
    pub h_movement : f32,
    pub v_movement : f32
}

#[derive(Component)]
pub struct Bullet {
    pub angle : f32
}

#[derive(Component)]
pub struct Enemy;

/////////////////////////////////////////////
// Rendering / animation

pub struct AnimationState {
    pub start_frame : usize,
    pub end_frame : usize,
    pub transitions : std::collections::HashMap<String, String>
}

// TODO: This struct should be an asset that can be shared across different sprites
pub struct AnimationTree {
    pub states : std::collections::HashMap<String, AnimationState>,

    // TODO: When this is turned into an asset, each entity will need to store these separately
    pub current_state : String
}

#[derive(Component)]
pub struct Animation {
    pub timer : Timer,
    // TODO: Turn this into an asset handle
    pub tree : AnimationTree
}