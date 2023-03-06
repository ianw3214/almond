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
//  - this crate can potentially be used to implement the asset loading
//  - https://docs.rs/bevy_common_assets/latest/bevy_common_assets/
pub struct AnimationTree {
    pub states : std::collections::HashMap<String, AnimationState>,

    // TODO: When this is turned into an asset, each entity will need to store these separately
    pub current_state : String
}

#[derive(Component)]
pub struct Animation {
    // TODO: This might want to be global so animations can update at the same time
    pub timer : Timer,
    // TODO: Turning this into an animationRequestComponent or something might be cleaner
    //  - Events might also be an option, need to do some research
    //  - https://bevy-cheatbook.github.io/programming/events.html
    pub events : Vec<String>,
    // TODO: Turn this into an asset handle
    pub tree : AnimationTree
}

#[derive (Component)]
pub struct RenderInfo {
    pub screen_width : f32,
    pub screen_height : f32
}