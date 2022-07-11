use specs::prelude::*;

use crate::components::*;
use crate::util::screen_to_grid_pos;
use crate::{ScreenInfo, CameraInfo, MouseCommand, GridSize, UIAction, CurrentAction};

use std::collections::VecDeque;

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        ReadExpect<'a, GridSize>,
        ReadExpect<'a, Option<MouseCommand>>,
        ReadExpect<'a, ScreenInfo>,
        ReadExpect<'a, CameraInfo>,
        WriteExpect<'a, VecDeque<UIAction>>,
        WriteExpect<'a, CurrentAction>,
        ReadStorage<'a, Selectable>,
        WriteStorage<'a, GridPosition>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        // process UI actions
        let ui_commands = &mut *data.4;
        let current_action = &mut *data.5;
        // TODO: Handle all ui commands instead of just 1 per tick
        if !ui_commands.is_empty() {
            let command = ui_commands.pop_back().unwrap();
            match command {
                UIAction::ActionButton(button) => {
                    if button == 0 {
                        *current_action = CurrentAction::Move;
                    }
                }
            }
        }

        let mouse_command = match &*data.1 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        match current_action {
            CurrentAction::Move => {
                // Handle movement
                match mouse_command {
                    &MouseCommand::Click(point) => {
                        let mouse_grid_pos = screen_to_grid_pos(&*data.2, &*data.3, &*data.0, point);
                        for (selectable, grid_pos) in (&data.6, &mut data.7).join() {
                            if selectable.selected {
                                grid_pos.x = mouse_grid_pos.x;
                                grid_pos.y = mouse_grid_pos.y;
                                *current_action = CurrentAction::None;
                                break
                            }
                        }
                    }
                }
            },
            CurrentAction::None => ()
        }
    }
}