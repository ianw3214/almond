use specs::prelude::*;

use crate::components::*;

use crate::MouseCommand;

pub struct Mouse;

impl<'a> System<'a> for Mouse {
    type SystemData = (
        ReadExpect<'a, Option<MouseCommand>>,
        ReadStorage<'a, Clickable>
    );

    fn run(&mut self, /* mut */ data: Self::SystemData) {
        let mouse_command = match &*data.0 {
            Some(mouse_command) => mouse_command,
            None => return
        };

        for _ in (&data.1).join() {
            println!("this is clickable!");
        }
        
        match mouse_command {
            &MouseCommand::Click(point) => {
                println!("clicked! point: {}, {}", point.x, point.y);
            }
        }
    }
}