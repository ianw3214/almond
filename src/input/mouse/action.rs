use specs::prelude::*;

use crate::components::*;
use crate::util::{screen_to_grid_pos, screen_to_world_pos};
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
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, Health>
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
                    if button == 1 {
                        *current_action = CurrentAction::Attack;
                    }
                }
            }
        }

        let mouse_command = match &*data.1 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        let grid = &*data.0;
        let screen = &*data.2;
        let camera = &*data.3;

        match current_action {
            CurrentAction::Move => {
                // Handle movement
                match mouse_command {
                    &MouseCommand::Click(point) => {
                        let mouse_grid_pos = screen_to_grid_pos(screen, camera, grid, point);
                        for (selectable, grid_pos) in (&data.6, &mut data.8).join() {
                            if selectable.selected {
                                grid_pos.x = mouse_grid_pos.x;
                                grid_pos.y = mouse_grid_pos.y;
                                *current_action = CurrentAction::None;
                                // TODO: Consume the moues action so it doesn't get further processed
                                break
                            }
                        }
                    }
                }
            },
            CurrentAction::Attack => {
                // Handle attack
                match mouse_command {
                    &MouseCommand::Click(point) => {
                        for (selectable, world_pos, health) in (&data.6, &data.7, (&mut data.9).maybe()).join() {
                            if let Some(health) = health {
                                let click_pos = screen_to_world_pos(screen, camera, point);
                                let x = click_pos.x;
                                let y = click_pos.y;
                                let sprite_x = world_pos.point.x + selectable.x_offset;
                                let sprite_y = world_pos.point.y + selectable.y_offset;
                                if x > sprite_x && x < sprite_x + selectable.width {
                                    if y > sprite_y && y < sprite_y + selectable.height {
                                        // TODO: Handle death...
                                        health.health = health.health - 1;
                                        *current_action = CurrentAction::None;
                                        // TODO: Consume the moues action so it doesn't get further processed
                                        break
                                    }
                                }
                            }
                        }
                    }
                }
            }
            CurrentAction::None => ()
        }
    }
}