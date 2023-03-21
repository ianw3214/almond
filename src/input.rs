use bevy::prelude::*;

const RIGHT_TRIGGER_THRESHOLD : f32 = 0.5;

#[derive(Default)]
pub struct KeyboardInputState {
    pub w_held : bool,
    pub a_held : bool,
    pub s_held : bool,
    pub d_held : bool
}

#[derive(Default)]
pub struct MouseInputState {
    pub mouse_released : bool,

    pub x : f32,
    pub y : f32,
    pub world_x : f32,
    pub world_y : f32
}

#[derive(Default)]
pub struct ControllerInputState {
    pub left_stick_x : f32,
    pub left_stick_y : f32,
    pub right_stick_x : f32,
    pub right_stick_y : f32,
    pub right_trigger : f32,
    // events
    pub right_trigger_pressed : bool,
    pub right_trigger_released : bool,
    // other state
    pub right_trigger_last_frame : f32
}

#[derive(Default, PartialEq)]
pub enum InputType {
    #[default] KEYBOARD,
    CONTROLLER
}

#[derive(Resource, Default)]
pub struct InputState {
    pub controller : ControllerInputState,
    pub keyboard : KeyboardInputState,
    pub mouse : MouseInputState,
    pub input_type : InputType
}

pub fn keyboard_events(
    keyboard_events : EventReader<bevy::input::keyboard::KeyboardInput>,
    mut input_state : ResMut<InputState>
) {
    /*
    for event in keyboard_events.iter() {
        println!("{:?}", event);
    }
     */
    if keyboard_events.len() > 0 {
        input_state.input_type = InputType::KEYBOARD;
    }
}

pub fn keyboard_system(
    keyboard_input : Res<Input<KeyCode>>,
    mut input_state : ResMut<InputState>
) {
    // reset all state first
    input_state.keyboard.w_held = false;
    input_state.keyboard.a_held = false;
    input_state.keyboard.s_held = false;
    input_state.keyboard.d_held = false;

    if keyboard_input.pressed(KeyCode::W) {
        input_state.keyboard.w_held = true;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input_state.keyboard.a_held = true;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input_state.keyboard.s_held = true;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input_state.keyboard.d_held = true;
    }

    /*
    if keyboard_input.just_pressed(KeyCode::A) {
        println!("A just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("A just released");
    }
     */
}

pub fn mouse_click_system(
    mouse_button_input : Res<Input<MouseButton>>,
    mut input_state : ResMut<InputState>
) {
    // reset all state first
    input_state.mouse.mouse_released = false;

    if mouse_button_input.just_released(MouseButton::Left) {
        input_state.mouse.mouse_released = true;
        input_state.input_type = InputType::KEYBOARD;
    }
}

pub fn mouse_position_system(
    windows : Res<Windows>,
    mut input_state : ResMut<InputState>
) {
    let window = windows.get_primary().unwrap();

    if let Some(position) = window.cursor_position() {
        input_state.mouse.x = position.x;
        input_state.mouse.y = position.y;
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
    // update previous state
    input_state.controller.right_trigger_last_frame = input_state.controller.right_trigger;
    // reset events
    input_state.controller.right_trigger_pressed = false;
    input_state.controller.right_trigger_released = false;
    // update current state based on actual gamepad inputs
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            println!("{:?} just pressed south", gamepad);
            input_state.input_type = InputType::CONTROLLER;
        }
        else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            println!("{:?} just released south", gamepad);
            input_state.input_type = InputType::CONTROLLER;
        }
        let right_trigger = button_axes
            .get(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() > 0.1 {
            input_state.controller.right_trigger = right_trigger;
            input_state.input_type = InputType::CONTROLLER;
        } else {
            input_state.controller.right_trigger = 0.0;
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.1 {
            input_state.controller.left_stick_x = left_stick_x;
            input_state.input_type = InputType::CONTROLLER;
        } else {
            input_state.controller.left_stick_x = 0.0;
        }

        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.1 {
            input_state.controller.left_stick_y = left_stick_y;
            input_state.input_type = InputType::CONTROLLER;
        }
        else {
            input_state.controller.left_stick_y = 0.0;
        }

        let right_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();
        if right_stick_x.abs() > 0.1 {
            input_state.controller.right_stick_x = right_stick_x;
            input_state.input_type = InputType::CONTROLLER;
        }
        else {
            input_state.controller.right_stick_x = 0.0;
        }

        let right_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
            .unwrap();
        if right_stick_y.abs() > 0.1 {
            input_state.controller.right_stick_y = right_stick_y;
            input_state.input_type = InputType::CONTROLLER;
        }
        else {
            input_state.controller.right_stick_y = 0.0;
        }
    }
    // update events based on state updates
    if input_state.controller.right_trigger >= RIGHT_TRIGGER_THRESHOLD && input_state.controller.right_trigger_last_frame < RIGHT_TRIGGER_THRESHOLD {
        input_state.controller.right_trigger_pressed = true;
        input_state.input_type = InputType::CONTROLLER;
    }
    if input_state.controller.right_trigger < RIGHT_TRIGGER_THRESHOLD && input_state.controller.right_trigger_last_frame >= RIGHT_TRIGGER_THRESHOLD {
        input_state.controller.right_trigger_released = true;
        input_state.input_type = InputType::CONTROLLER;
    }
}