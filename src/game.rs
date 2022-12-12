use bevy::prelude::*;

use crate::input;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movement {
    pub h_movement : f32,
    pub v_movement : f32
}

fn handle_movement(mut query : Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in &mut query {
        transform.translation.x = transform.translation.x + movement.h_movement;
        transform.translation.y = transform.translation.y + movement.v_movement;
    }
}

fn setup_camera(mut commands : Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_player(mut commands : Commands) {
    commands.spawn(SpriteBundle {
        sprite : Sprite { 
            color: Color::rgb(0.6, 0.6, 0.6), 
            ..default()
        },
        transform : Transform {
            scale : Vec3::new(10.0, 10.0, 10.0),
            translation : Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).insert(Player)
    .insert(Movement {
        h_movement : 1.0,
        v_movement : 0.0
    });
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app : &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(add_player)
            .add_system(input::keyboard_events)
            .add_system(input::keyboard_system)
            .add_system(input::gamepad_events)
            .add_system(input::gamepad_system)
            .add_system(handle_movement);
    }
}