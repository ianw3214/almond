use bevy::prelude::*;

use crate::game::components::*;
use crate::input;

fn update_player_movement(mut query : Query<&mut Movement, With<Player>>, input_state : Res<input::InputState>) {
    for mut movement in &mut query {
        movement.h_movement = input_state.controller.left_stick_x;
        movement.v_movement = input_state.controller.left_stick_y;
    }
}

fn update_bullet_movement(mut query : Query<(&mut Movement, &Bullet)>) {
    for (mut movement, bullet) in &mut query {
        const BULLET_SPEED : f32 = 2.0;
        let h_movement = BULLET_SPEED * bullet.angle.cos();
        let v_movement = BULLET_SPEED * bullet.angle.sin();
        movement.h_movement = h_movement;
        movement.v_movement = v_movement;
    }    
}

fn handle_bullet_collision(
    mut commands : Commands,
    bullets : Query<(Entity, &Transform), With<Bullet>>,
    targets : Query<(Entity, &Transform), With<Enemy>>) 
{
    for (bullet, b) in bullets.iter() {
        for (target, t) in targets.iter() {
            if b.translation.x < t.translation.x + t.scale.x && b.translation.x + b.scale.x > t.translation.x {
                if b.translation.y < t.translation.y + t.scale.y && b.translation.y + b.scale.y > t.translation.y { 
                    commands.entity(bullet).despawn();
                    commands.entity(target).despawn();
                }
            }
        }
    }
}

fn handle_movement(mut query : Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in &mut query {
        transform.translation.x = transform.translation.x + movement.h_movement;
        transform.translation.y = transform.translation.y + movement.v_movement;
    }
}

fn spawn_bullet(
    mut commands : Commands, 
    players : Query<&Transform, With<Player>>,
    input_state : Res<input::InputState>) 
{
    if input_state.controller.right_trigger_released {
        // TODO: This should depend on player facing instead of held stick angle
        let mut angle = 0.0;
        if input_state.controller.right_stick_x != 0.0 || input_state.controller.right_stick_y != 0.0 {
            angle = input_state.controller.right_stick_y.atan2(input_state.controller.right_stick_x);
        }
        for player in players.iter() {
            commands.spawn(SpriteBundle{
                sprite : Sprite {
                    color : Color::rgb(0.8, 0.5, 0.5),
                    ..default()
                },
                transform : Transform {
                    scale : Vec3::new(3.0, 3.0, 3.0),
                    translation : player.translation,
                    ..default()
                },
                ..default()
            }).insert(Bullet {
                angle : angle,
            }).insert(Movement::default());   
        }
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
            .add_system(handle_bullet_collision)
            .add_system(spawn_bullet)
            .add_system(handle_movement);
    }
}