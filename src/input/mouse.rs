use specs::prelude::*;

use crate::{components::*, util::screen_to_window_pos, ScreenInfo};

use crate::MouseCommand;

pub struct Mouse;

impl<'a> System<'a> for Mouse {
    type SystemData = (
        ReadExpect<'a, Option<MouseCommand>>,
        ReadExpect<'a, ScreenInfo>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Clickable>
    );

    fn run(&mut self, mut  data: Self::SystemData) {
        let mouse_command = match &*data.0 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        match mouse_command {
            &MouseCommand::Click(point) => {
                for clickable in (&mut data.3).join() {
                    clickable.selected = false;
                }

                let world_pos = screen_to_window_pos(&*data.1, point);
                let x = world_pos.x;
                let y = world_pos.y;
                for (pos, click) in (&data.2, &mut data.3).join() {
                    if x > pos.0.x && x < pos.0.x + click.width {
                        if y > pos.0.y && y < pos.0.y + click.height {
                            click.selected = true;
                            return
                        }
                    }
                }
            }
        }
    }
}