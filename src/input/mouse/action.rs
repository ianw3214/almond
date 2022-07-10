use specs::prelude::*;

use crate::components::*;
use crate::util::screen_to_world_pos;
use crate::{ScreenInfo, CameraInfo, MouseCommand, GridSize};

pub struct Action;

impl<'a> System<'a> for Action {
    type SystemData = (
        ReadExpect<'a, GridSize>,
        ReadExpect<'a, Option<MouseCommand>>,
        ReadExpect<'a, ScreenInfo>,
        ReadExpect<'a, CameraInfo>,
        ReadStorage<'a, Selectable>,
        WriteStorage<'a, GridPosition>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let mouse_command = match &*data.1 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        match mouse_command {
            &MouseCommand::Click(point) => {
                let world_pos = screen_to_world_pos(&*data.2, &*data.3, point);
                let grid_x = world_pos.x / data.0.width;
                let grid_y = world_pos.y / data.0.height;
                for (selectable, grid_pos) in (&data.4, &mut data.5).join() {
                    if selectable.selected {
                        grid_pos.x = grid_x;
                        grid_pos.y = grid_y;
                        break
                    }
                }
            }
        }
    }
}