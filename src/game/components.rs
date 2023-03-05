use bevy::prelude::*;

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

// Rendering / animation
#[derive(Component)]
pub struct AnimationTimer(pub Timer);