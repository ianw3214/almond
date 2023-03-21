use bevy::prelude::*;

use crate::game::components::*;
use crate::input;

use crate::game::graphics;

fn set_mouse_world_coordinates(
    mut input_state : ResMut<input::InputState>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let viewport_position : Vec2 = Vec2::new(input_state.mouse.x, input_state.mouse.y);
    let (camera, camera_transform) = camera_q.single();
    let world_pos = camera.viewport_to_world(camera_transform, viewport_position)
        .map(|ray| ray.origin.truncate()).unwrap();
    input_state.mouse.world_x = world_pos.x;
    input_state.mouse.world_y = world_pos.y;
}

fn update_player_movement(mut query : Query<(&mut Movement, &mut Animation), With<Player>>, input_state : Res<input::InputState>) {
    for (mut movement, mut anim) in &mut query {
        if input_state.input_type == input::InputType::CONTROLLER {
            movement.h_movement = input_state.controller.left_stick_x;
            movement.v_movement = input_state.controller.left_stick_y;
        }
        if input_state.input_type == input::InputType::KEYBOARD {
            let mut h_movement = 0.0;
            let mut v_movement = 0.0;
            if input_state.keyboard.w_held {
                v_movement += 1.0;
            }
            if input_state.keyboard.a_held {
                h_movement -= 1.0;
            }
            if input_state.keyboard.s_held {
                v_movement -= 1.0;
            }
            if input_state.keyboard.d_held {
                h_movement += 1.0;
            }
            if h_movement != 0.0 && v_movement != 0.0 {
                v_movement *= 0.70710678118;
                h_movement *= 0.70710678118;
            }
            movement.h_movement = h_movement;
            movement.v_movement = v_movement;
        }
        let angle = movement.v_movement.atan2(movement.h_movement);
        if movement.h_movement != 0.0 || movement.v_movement != 0.0 {
            if angle > std::f32::consts::FRAC_PI_4 * 3.0 || angle < -std::f32::consts::FRAC_PI_4 * 3.0 {
                anim.events.push(String::from("left"));
            } else {
                if angle > std::f32::consts::FRAC_PI_4 {
                    anim.events.push(String::from("up"));
                } else if angle < -std::f32::consts::FRAC_PI_4 {
                    anim.events.push(String::from("down"));
                } else {
                    anim.events.push(String::from("right"));
                }
            }
        }
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
    let mut bullet_angle : Option<f32> = None;
    if input_state.input_type == input::InputType::CONTROLLER && input_state.controller.right_trigger_released {
        if input_state.controller.right_stick_x != 0.0 || input_state.controller.right_stick_y != 0.0 {
            bullet_angle = Some(input_state.controller.right_stick_y.atan2(input_state.controller.right_stick_x));
        } else {
            bullet_angle = Some(0.0);
        }
    }
    if input_state.input_type == input::InputType::KEYBOARD && input_state.mouse.mouse_released {
        for player in players.iter() {
            // TODO: The transform position has to be translated between screen/world positions
            let x_offset = input_state.mouse.world_x - player.x;
            let y_offset = input_state.mouse.world_y - player.y;
            if x_offset != 0.0 || y_offset != 0.0 {
                bullet_angle = Some(y_offset.atan2(x_offset));
            } else {
                bullet_angle = Some(0.0);
            }
        }
    }
    if let Some(angle) = bullet_angle {
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
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    ));
}

fn add_player(
    mut commands : Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    anim_trees : Res<graphics::AnimationTreeHandles>) 
{
    let frame_width : f32 = 10.0;
    let frame_height : f32 = 10.0;
    let texture_handle = asset_server.load("test.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(frame_width, frame_height), 4, 4, Some(Vec2::new(1.0, 1.0)), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(SpriteSheetBundle {
        texture_atlas : texture_atlas_handle,
        ..default()
    }).insert(Player)
    .insert(Animation{
        timer : Timer::from_seconds(0.1, TimerMode::Repeating),
        events : Vec::new(),
        current_state : String::new(),
        tree : anim_trees.handle_map.get("player").unwrap().clone()
    })
    .insert(RenderInfo {
        screen_width : 100.0,
        screen_height : 100.0
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

pub struct Game;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum OrderLabel {
    /// everything that handles input
    Input,
    /// Certain input to game translations
    InputStaging,
    /// everything that moves things (works with transforms)
    GameState,
    /// systems that affect rendering
    Rendering,
}

impl Plugin for Game {
    fn build(&self, app : &mut App) {
        app.init_resource::<input::InputState>()
            .init_resource::<graphics::AnimationTreeHandles>()
            .add_asset::<AnimationTree>()
            .add_startup_system(graphics::initialize_anim_trees
                .before(add_player))
            .add_startup_system(setup_camera)
            .add_startup_system(add_player)
            .add_startup_system(add_enemy)
            .add_system(graphics::fixup_sprites)
            .add_system(graphics::initialize_anim_states)
            // .add_system(input::gamepad_events)
            .add_system_set(
                SystemSet::new()
                    .label(OrderLabel::Input)
                    .with_system(input::keyboard_events)
                    .with_system(input::keyboard_system)
                    .with_system(input::mouse_click_system)
                    .with_system(input::mouse_position_system)
                    .with_system(input::gamepad_system)
            ).add_system_set(
                SystemSet::new()
                    .label(OrderLabel::InputStaging)
                    .after(OrderLabel::Input)
                    .before(OrderLabel::GameState)
                    .with_system(set_mouse_world_coordinates)
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
                    .with_system(graphics::update_sprite_translation)
                    .with_system(graphics::update_sprite_animation)
                    .with_system(graphics::update_sprite_size)
            );
    }
}