use specs::prelude::*;

use crate::components::*;

use crate::MouseCommand;

pub struct Mouse;

impl<'a> System<'a> for Mouse {
    type SystemData = (
        ReadExpect<'a, Option<MouseCommand>>,
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
                for clickable in (&mut data.2).join() {
                    clickable.selected = false;
                }

                let x = point.x;
                let y = point.y;
                for (pos, click) in (&data.1, &mut data.2).join() {
                    println!("{} {} {} {}", x, y, pos.0.x, pos.0.y);
                    if x > pos.0.x && x < pos.0.x + click.width {
                        println!("TEST2");
                        if y > pos.0.y && y < pos.0.y + click.height {
                            println!("TEST3");
                            click.selected = true;
                            return
                        }
                    }
                }
            }
        }
    }
}