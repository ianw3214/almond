use specs::prelude::*;

use crate::components::*;

pub struct Pathfinder;

impl<'a> System<'a> for Pathfinder {
    type SystemData = (
        WriteStorage<'a, Movement>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        for (movement, pos) in (&mut data.0, &mut data.1).join() {
            if let Some(target) = movement.target {
                let x = target.0;
                let y = target.1;
                if pos.x < x {
                    pos.x = pos.x + movement.speed;
                    if pos.x > x {
                        pos.x = x;
                    }
                }
                if pos.x > x {
                    pos.x = pos.x - movement.speed;
                    if pos.x < x {
                        pos.x = x;
                    }
                }
                if pos.y < y {
                    pos.y = pos.y + movement.speed;
                    if pos.y > y {
                        pos.y = y;
                    }
                }
                if pos.y > y {
                    pos.y = pos.y - movement.speed;
                    if pos.y < y {
                        pos.y = y;
                    }
                }
                if pos.x == x && pos.y == y {
                    movement.target = None;
                }
            }
        }
    }
}