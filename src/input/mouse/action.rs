use specs::prelude::*;

use crate::components::*;
use crate::util::screen_to_grid_pos;
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
                let mouse_grid_pos = screen_to_grid_pos(&*data.2, &*data.3, &*data.0, point);
                for (selectable, grid_pos) in (&data.4, &mut data.5).join() {
                    if selectable.selected {
                        grid_pos.x = mouse_grid_pos.x;
                        grid_pos.y = mouse_grid_pos.y;
                        break
                    }
                }
            }
        }
    }
}