use specs::prelude::*;

use crate::components::*;
use crate::util::{screen_to_grid_pos, screen_to_world_pos};
use crate::{ScreenInfo, CameraInfo, MouseCommand, GridSize, UIAction, CurrentAction, SelectedEntity};

use std::collections::VecDeque;

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        // Global resources
        ReadExpect<'a, GridSize>,
        ReadExpect<'a, Option<MouseCommand>>,
        ReadExpect<'a, ScreenInfo>,
        ReadExpect<'a, CameraInfo>,
        WriteExpect<'a, VecDeque<UIAction>>,
        WriteExpect<'a, CurrentAction>,
        WriteExpect<'a, SelectedEntity>,
        // Components
        ReadStorage<'a, Selectable>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Turn>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        // Global resources
        let grid_size = &*data.0;
        let mouse_command = &*data.1;
        let screen_info = &*data.2;
        let camera_info = &*data.3;
        let ui_actions = &mut *data.4;
        let current_action = &mut *data.5;
        let selected_entity = &mut *data.6;
        // Components
        let selectables = &data.7;
        let world_positions = &data.8;
        let grid_positions = &mut data.9;
        let healths = &mut data.10;
        let turns = &mut data.11;

        // process UI actions
        // TODO: Handle all ui commands instead of just 1 per tick
        if !ui_actions.is_empty() {
            let command = ui_actions.pop_back().unwrap();
            match command {
                UIAction::ActionButton(button) => {
                    // TODO: Potentially separate out movement button
                    if button == 0 {
                        *current_action = CurrentAction::Move;
                    }
                    else {
                        *current_action = CurrentAction::Attack(button - 1);
                    }
                }
            }
        }

        let mouse_command = match mouse_command {
            Some(mouse_command) => mouse_command,
            None => return
        };

        // TODO: Is selected entity somehow not the same as current turn entity?
        //  - Need to handle differently if so
        match current_action {
            CurrentAction::Move => {
                // Handle movement
                match mouse_command {
                    &MouseCommand::Click(point) => {
                        let mouse_grid_pos = screen_to_grid_pos(screen_info, camera_info, grid_size, point);
                        match selected_entity.0 {
                            Some(selected) => {
                                let grid_pos = grid_positions.get_mut(selected).unwrap();
                                let turn = turns.get_mut(selected).unwrap();
                                // TODO: Handle verifying valid movement target
                                //  - move range
                                //  - validate empty spot/valid path
                                grid_pos.x = mouse_grid_pos.x;
                                grid_pos.y = mouse_grid_pos.y;
                                *current_action = CurrentAction::None;
                                // end entity turn
                                turn.current = false;
                                // TODO: Consume the moues action so it doesn't get further processed
                            },
                            None => ()
                        }
                    }
                }
            },
            CurrentAction::Attack(index) => {
                // Handle attack
                match mouse_command {
                    &MouseCommand::Click(point) => {
                        for (selectable, world_pos, health) in (selectables, world_positions, healths.maybe()).join() {
                            if let Some(health) = health {
                                let click_pos = screen_to_world_pos(screen_info, camera_info, point);
                                let x = click_pos.x;
                                let y = click_pos.y;
                                let sprite_x = world_pos.point.x + selectable.x_offset;
                                let sprite_y = world_pos.point.y + selectable.y_offset;
                                if x > sprite_x && x < sprite_x + selectable.width {
                                    if y > sprite_y && y < sprite_y + selectable.height {
                                        match selected_entity.0 {
                                            Some(selected) => {
                                                let turn = turns.get_mut(selected).unwrap();
                                                let damage = turn.attacks[*index as usize].damage;
                                                // TODO: Handle death...
                                                health.health = health.health - damage;
                                                *current_action = CurrentAction::None;
                                                // end entity turn
                                                turn.current = false;
                                            },
                                            None => ()
                                        }
                                        // TODO: Consume the moues action so it doesn't get further processed
                                        break
                                    }
                                }
                                // If no valid input was matched, reset the current action
                                *current_action = CurrentAction::None;
                                break
                            }
                        }
                    }
                }
            }
            CurrentAction::None => ()
        }
    }
}