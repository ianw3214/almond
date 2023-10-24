use bevy::{prelude::*, reflect::TypeUuid};

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

#[derive(Component)]
pub struct Collision {
    pub width : f32,
    pub height : f32
}

/////////////////////////////////////////////
// Rendering / animation

#[derive(Debug)]
pub struct AnimationState {
    pub start_frame : usize,
    pub end_frame : usize,
    pub transitions : std::collections::HashMap<String, String>
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct AnimationTree {
    pub states : std::collections::HashMap<String, AnimationState>,
    pub initial : String
}

#[derive(Component)]
pub struct Animation {
    // TODO: This might want to be global so animations can update at the same time
    pub timer : Timer,
    // TODO: Turning this into an animationRequestComponent or something might be cleaner
    //  - Events might also be an option, need to do some research
    //  - https://bevy-cheatbook.github.io/programming/events.html
    pub events : Vec<String>,

    pub current_state : String,
    pub tree : Handle<AnimationTree>
}

#[derive (Component)]
pub struct RenderInfo {
    pub screen_width : f32,
    pub screen_height : f32,
    pub x_offset : f32,
    pub y_offset : f32
}

//////////////////////////////////////////////
/// AI components
#[derive(Component, Default)]
pub struct RandomMovement {
    pub cooldown : u32,
    pub direction : f32
}

//////////////////////////////////////////////
// Other misc. components

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;