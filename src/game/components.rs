use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

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