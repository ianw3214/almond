use std::collections::HashMap;

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
    bullets : Query<(Entity, &WorldPosition, &Transform), With<Bullet>>,
    targets : Query<(Entity, &WorldPosition, &Transform), With<Enemy>>) 
{
    for (bullet, b_pos, b) in bullets.iter() {
        for (target, t_pos, t) in targets.iter() {
            if b_pos.x < t_pos.x + t.scale.x && b_pos.x + b.scale.x > t_pos.x {
                if b_pos.y < t_pos.y + t.scale.y && b_pos.y + b.scale.y > t_pos.y { 
                    commands.entity(bullet).despawn();
                    commands.entity(target).despawn();
                }
            }
        }
    }
}

fn handle_movement(mut query : Query<(&Movement, &mut WorldPosition)>) {
    for (movement, mut position) in &mut query {
        position.x = position.x + movement.h_movement;
        position.y = position.y + movement.v_movement;
    }
}

fn spawn_bullet(
    mut commands : Commands, 
    players : Query<&WorldPosition, With<Player>>,
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
                // The transform translations have to be properly set on spawn for now
                // This might eventually become some sort of flagging/initizliation system
                transform : Transform {
                    scale : Vec3::new(3.0, 3.0, 3.0),
                    translation : Vec3::new(player.x, player.y, 0.0),
                    ..default()
                },
                ..default()
            }).insert(Bullet {
                angle : angle,
            }).insert(WorldPosition{ 
                x : player.x,
                y : player.y
            }).insert(Movement::default());
        }
    }
}

fn setup_camera(mut commands : Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_player(
    mut commands : Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) 
{
    let mut up_transitions = HashMap::new();
    up_transitions.insert(String::from("down"), String::from("down"));
    up_transitions.insert(String::from("left"), String::from("left"));
    up_transitions.insert(String::from("right"), String::from("right"));
    let up_state = AnimationState {
        start_frame : 0,
        end_frame : 3,
        transitions : up_transitions
    };

    let mut down_transitions = HashMap::new();
    down_transitions.insert(String::from("up"), String::from("up"));
    down_transitions.insert(String::from("left"), String::from("left"));
    down_transitions.insert(String::from("right"), String::from("right"));
    let down_state = AnimationState {
        start_frame : 0,
        end_frame : 3,
        transitions : down_transitions
    };

    let mut left_transitions = HashMap::new();
    left_transitions.insert(String::from("up"), String::from("up"));
    left_transitions.insert(String::from("down"), String::from("down"));
    left_transitions.insert(String::from("right"), String::from("right"));
    let left_state = AnimationState {
        start_frame : 0,
        end_frame : 3,
        transitions : left_transitions
    };

    let mut right_transitions = HashMap::new();
    right_transitions.insert(String::from("up"), String::from("up"));
    right_transitions.insert(String::from("down"), String::from("down"));
    right_transitions.insert(String::from("left"), String::from("left"));
    let right_state = AnimationState {
        start_frame : 0,
        end_frame : 3,
        transitions : right_transitions
    };

    let mut player_animation_tree = HashMap::new();
    player_animation_tree.insert(String::from("up"), up_state);
    player_animation_tree.insert(String::from("down"), down_state);
    player_animation_tree.insert(String::from("left"), left_state);
    player_animation_tree.insert(String::from("right"), right_state);

    let texture_handle = asset_server.load("test.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(10.0, 10.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(SpriteSheetBundle {
        texture_atlas : texture_atlas_handle,
        ..default()
    }).insert(Player)
    .insert(Animation{
        timer : Timer::from_seconds(0.1, TimerMode::Repeating),
        tree : AnimationTree {
            states : player_animation_tree,
            current_state : String::from("right")
        }
    })
    .insert(WorldPosition{ x : 0.0, y : 0.0})
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
            translation : Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    }).insert(Enemy)
    .insert(WorldPosition{ x : 100.0, y : 0.0});
}

fn update_sprite_translation(mut sprites : Query<(&mut Transform, &WorldPosition)>,) {
    for (mut transform, position) in sprites.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn update_sprite_animation(
    time : Res<Time>,
    mut query : Query<(&mut Animation, &mut TextureAtlasSprite)>) 
{
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            let next_index = sprite.index + 1;
            let curr_state = anim.tree.states.get(&anim.tree.current_state).unwrap();
            sprite.index = if next_index > curr_state.end_frame { curr_state.start_frame } else { next_index };
        }
    }
}

pub struct Game;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum OrderLabel {
    /// everything that handles input
    Input,
    /// everything that moves things (works with transforms)
    GameState,
    /// systems that affect rendering
    Rendering,
}

impl Plugin for Game {
    fn build(&self, app : &mut App) {
        app.init_resource::<input::InputState>()
            .add_startup_system(setup_camera)
            .add_startup_system(add_player)
            .add_startup_system(add_enemy)
            // .add_system(input::gamepad_events)
            .add_system_set(
                SystemSet::new()
                    .label(OrderLabel::Input)
                    .with_system(input::keyboard_events)
                    .with_system(input::keyboard_system)
                    .with_system(input::gamepad_system)
            ).add_system_set(
                SystemSet::new()
                    .label(OrderLabel::GameState)
                    .after(OrderLabel::Input)
                    .before(OrderLabel::Rendering)
                    .with_system(update_player_movement)
                    .with_system(update_bullet_movement)
                    .with_system(handle_bullet_collision)
                    .with_system(spawn_bullet)
                    .with_system(handle_movement)
            )
            .add_system_set(
                SystemSet::new()
                    .label(OrderLabel::Rendering)
                    .after(OrderLabel::GameState)
                    .with_system(update_sprite_translation)
                    .with_system(update_sprite_animation)
            );
    }
}