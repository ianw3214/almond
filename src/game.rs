use bevy::prelude::*;

use crate::input;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Movement {
    pub h_movement : f32,
    pub v_movement : f32
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Enemy;

fn update_player_movement(mut query : Query<&mut Movement, With<Player>>, input_state : Res<input::InputState>) {
    for mut movement in &mut query {
        movement.h_movement = input_state.controller.left_stick_x;
        movement.v_movement = input_state.controller.left_stick_y;
    }
}

fn update_bullet_movement(mut query : Query<&mut Movement, With<Bullet>>) {
    for mut movement in &mut query {
        movement.h_movement = 2.0;
    }    
}

fn handle_movement(mut query : Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in &mut query {
        transform.translation.x = transform.translation.x + movement.h_movement;
        transform.translation.y = transform.translation.y + movement.v_movement;
    }
}

fn spawn_bullet(mut commands : Commands, input_state : Res<input::InputState>) {
    if input_state.controller.right_trigger_released {
        commands.spawn(SpriteBundle{
            sprite : Sprite {
                color : Color::rgb(0.8, 0.5, 0.5),
                ..default()
            },
            transform : Transform {
                scale : Vec3::new(3.0, 3.0, 3.0),
                translation : Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        }).insert(Bullet)
        .insert(Movement::default());
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
    .insert(Movement::default());
}

fn add_enemy(mut commands : Commands) {
    commands.spawn(SpriteBundle{
        sprite : Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            ..default()
        },
        transform : Transform {
            scale : Vec3::new(10.0, 10.0, 10.0),
            translation : Vec3::new(100.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).insert(Enemy);
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app : &mut App) {
        app.init_resource::<input::InputState>()
            .add_startup_system(setup_camera)
            .add_startup_system(add_player)
            .add_startup_system(add_enemy)
            .add_system(input::keyboard_events)
            .add_system(input::keyboard_system)
            // .add_system(input::gamepad_events)
            .add_system(input::gamepad_system)
            .add_system(update_player_movement)
            .add_system(update_bullet_movement)
            .add_system(spawn_bullet)
            .add_system(handle_movement);
    }
}