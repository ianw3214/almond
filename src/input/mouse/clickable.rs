use specs::prelude::*;

use crate::{components::*, util::screen_to_world_pos, ScreenInfo, CameraInfo, SelectedEntity};

use crate::MouseCommand;

pub struct Clickable;

impl<'a> System<'a> for Clickable {
    type SystemData = (
        // Global resources
        ReadExpect<'a, Option<MouseCommand>>,
        ReadExpect<'a, ScreenInfo>,
        ReadExpect<'a, CameraInfo>,
        WriteExpect<'a, SelectedEntity>,
        // List of all entities
        Entities<'a>,
        // Components
        ReadStorage<'a, WorldPosition>,
        WriteStorage<'a, Selectable>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        // Global resources
        let mouse_command = &*data.0;
        let screen_info = &*data.1;
        let camera_info = &*data.2;
        let selected_entity = &mut *data.3;
        // List of all entities
        let entities = &*data.4;
        // Components
        let world_positions = &data.5;
        let selectables = &mut data.6;

        let mouse_command = match mouse_command {
            Some(mouse_command) => mouse_command,
            None => return
        };

        match mouse_command {
            &MouseCommand::Click(point) => {
                selected_entity.0 = Option::None;

                let click_pos = screen_to_world_pos(screen_info, camera_info, point);
                let x = click_pos.x;
                let y = click_pos.y;
                for (entity, pos, selectable) in (entities, world_positions, selectables).join() {
                    let sprite_x = pos.point.x + selectable.x_offset;
                    let sprite_y = pos.point.y + selectable.y_offset;
                    if x > sprite_x && x < sprite_x + selectable.width {
                        if y > sprite_y && y < sprite_y + selectable.height {
                            selected_entity.0 = Some(entity);
                            return
                        }
                    }
                }
            }
        }
    }
}