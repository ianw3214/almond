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
        let screen = &*data.1;
        let camera = &*data.2;

        let mouse_command = match &*data.0 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        match mouse_command {
            &MouseCommand::Click(point) => {
                for selectable in (&mut data.4).join() {
                    selectable.selected = false;
                }

                let click_pos = screen_to_world_pos(screen, camera, point);
                let x = click_pos.x;
                let y = click_pos.y;
                for (pos, selectable) in (&data.3, &mut data.4).join() {
                    let sprite_x = pos.point.x + selectable.x_offset;
                    let sprite_y = pos.point.y + selectable.y_offset;
                    if x > sprite_x && x < sprite_x + selectable.width {
                        if y > sprite_y && y < sprite_y + selectable.height {
                            selectable.selected = true;
                            return
                        }
                    }
                }
            }
        }
    }
}