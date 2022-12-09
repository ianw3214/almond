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

fn gamepad_system(
    gamepads : Res<Gamepads>,
    button_inputs : Res<Input<GamepadButton>>,
    button_axes : Res<Axis<GamepadButton>>,
    axes : Res<Axis<GamepadAxis>>
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            println!("{:?} just pressed south", gamepad);
        }
        else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            println!("{:?} just released south", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() > 0.1 {
            println!("{:?} right trigger 2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.1 {
            println!("{:?} left stick x value is {}", gamepad, left_stick_x);
        }
    }
}

fn gamepad_events(mut gamepad_event : EventReader<GamepadEvent>) {
    for event in gamepad_event.iter() {
        match event.event_type {
            GamepadEventType::Connected(_) => {
                println!("{:?} connected", event.gamepad)
            },
            GamepadEventType::Disconnected => {
                println!("{:?} disconnected", event.gamepad)
            },
            GamepadEventType::ButtonChanged(button_type, value) => {
                println!("{:?} of {:?} is changed to {}", button_type, event.gamepad, value)
            },
            GamepadEventType::AxisChanged(axis_type, value) => {
                println!("{:?} of {:?} is changed to {}", axis_type, event.gamepad, value)
            }
        }
    }
}

pub struct Game;

impl Plugin for Game {
    fn build(&self, app : &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(add_player)
            .add_system(gamepad_events)
            .add_system(gamepad_system);
    }
}