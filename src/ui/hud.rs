use specs::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

use std::collections::VecDeque;

use crate::{components::*, SelectedEntity};
use crate::{ScreenInfo, MouseInfo, MouseCommand, UIAction};

pub type SystemData<'a> = (
    // Global resources
    ReadExpect<'a, ScreenInfo>,
    ReadExpect<'a, MouseInfo>,
    ReadExpect<'a, SelectedEntity>,
    WriteExpect<'a, Option<MouseCommand>>,
    WriteExpect<'a, VecDeque<UIAction>>,
    // Components
    ReadStorage<'a, Selectable>,
    ReadStorage<'a, Turn>,
    ReadStorage<'a, Health>
);

pub fn run(mut data: SystemData) {
    // Global resources
    let screen_info = &*data.0;
    let _mouse_info = &*data.1;
    let selected_entity = &*data.2;
    let mouse_command = &mut *data.3;
    let ui_actions = &mut *data.4;
    // Components
    let _selectables = &data.5;
    let turns = &data.6;
    let _healths = &data.7;

    let found_mouse_command = match mouse_command {
        Some(command) => command,
        None => return
    };

    match selected_entity.0 {
        Some(selected) => {
            let mut mouse_button_handled = false;
            // TODO: This should only be handled when an entity w/ actions is selected
            match found_mouse_command {
                MouseCommand::Click(point) => {
                    let turn = turns.get(selected);
                    if let Some(turn) = turn {
                        for (index, _attack) in turn.actions.iter().enumerate() {
                            let x = screen_info.width - 80 - (80 * index as i32);
                            let y = screen_info.height - 80;
                            if (point.x > x && point.x < x + 60) && (point.y > y && point.y < y + 60) {
                                ui_actions.push_back(UIAction::ActionButton(index as i32));
                                // Consume the click so it doesn't get processed by other systems
                                mouse_button_handled = true;
                            }
                        }
                    }
                }
            }
            if mouse_button_handled {
                *mouse_command = Option::None;
            }
        },
        None => ()
    }
}

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &mut [Texture],
    data: SystemData
) -> Result<(), String> {
    // Global resources
    let screen_info = &*data.0;
    let mouse_info = &*data.1;
    let selected_entity = &*data.2;
    let _mouse_command = &*data.3;
    let _ui_actions = &*data.4;
    // Components
    let _selectables = &data.5;
    let turns = &data.6;
    let healths = &data.7;

    // Only draw actions if selected unit has turn
    match selected_entity.0 {
        Some(selected) => {
            // TODO: Refactor the rendering of icons to a shared function
            // TODO: Need to check the valid actions for the selected entity
            let turn = turns.get(selected);
            if let Some(turn) = turn {
                for (index, _attack) in turn.actions.iter().enumerate() {
                    // TODO: Actions themselves should store which icon to render
                    //  - Remove hard coded icon choice and get from data
                    let icon = if index == 0 {0} else {3};

                    let x = screen_info.width - 80 - (80 * index as i32);
                    let y = screen_info.height - 80;
                    let screen_rect = Rect::new(x, y, 60, 60);
                    if (mouse_info.x > x && mouse_info.x < x + 60) && (mouse_info.y > y && mouse_info.y < y + 60) {
                        textures[icon].set_color_mod(150, 150, 150);
                    }
                    else {
                        textures[icon].set_color_mod(255, 255, 255);
                    }
                    canvas.copy(&textures[icon], None, screen_rect)?;
                }
            }

            // Health of current selected entity
            let health = healths.get(selected);
            if let Some(health) = health {
                let y = screen_info.height - 80;
                for index in 1..health.max_health {
                    let screen_rect = Rect::new(index * 40 + 80, y, 30, 30);
                    let texture_id = if index < health.health { 1 } else { 2 };
                    canvas.copy(&textures[texture_id], None, screen_rect)?;
                }
            }
        },
        _ => ()
    }

    Ok(())
}