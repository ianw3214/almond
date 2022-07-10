use specs::prelude::*;

use crate::{components::*, util::screen_to_world_pos, ScreenInfo, CameraInfo};

use crate::MouseCommand;

pub struct Clickable;

impl<'a> System<'a> for Clickable {
    type SystemData = (
        ReadExpect<'a, Option<MouseCommand>>,
        ReadExpect<'a, ScreenInfo>,
        ReadExpect<'a, CameraInfo>,
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Selectable>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let mouse_command = match &*data.0 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        match mouse_command {
            &MouseCommand::Click(point) => {
                for selectable in (&mut data.4).join() {
                    selectable.selected = false;
                }

                let world_pos = screen_to_world_pos(&*data.1, &*data.2, point);
                let x = world_pos.x;
                let y = world_pos.y;
                for (pos, click) in (&data.3, &mut data.4).join() {
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