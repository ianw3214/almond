use bevy::prelude::*;

#[derive(Default)]
pub struct ControllerInputState {
    pub left_stick_x : f32,
    pub left_stick_y : f32
}

#[derive(Resource, Default)]
pub struct InputState {
    pub controller : ControllerInputState
}

pub fn keyboard_events(mut keyboard_events : EventReader<bevy::input::keyboard::KeyboardInput>) {
    for event in keyboard_events.iter() {
        println!("{:?}", event);
    }
}

pub fn keyboard_system(keyboard_input : Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("A currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("A just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("A just released");
    }
}

/*
pub fn gamepad_events(mut gamepad_event : EventReader<GamepadEvent>) {
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
 */

pub fn gamepad_system(
    gamepads : Res<Gamepads>,
    button_inputs : Res<Input<GamepadButton>>,
    button_axes : Res<Axis<GamepadButton>>,
    axes : Res<Axis<GamepadAxis>>,
    mut input_state : ResMut<InputState>
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
            input_state.controller.left_stick_x = left_stick_x;
        } else {
            input_state.controller.left_stick_x = 0.0;
        }

        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.1 {
            input_state.controller.left_stick_y = left_stick_y;
        }
        else {
            input_state.controller.left_stick_y = 0.0;
        }
    }
}