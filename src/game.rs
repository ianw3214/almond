use bevy::prelude::*;

#[derive(Component)]
struct Player;

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
            ..default()
        },
        ..default()
    }).insert(Player);
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app : &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(add_player);
    }
}