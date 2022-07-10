use specs::prelude::*;

use crate::components::*;

pub struct TurnSystem;

impl<'a> System<'a> for TurnSystem {
    type SystemData = WriteStorage<'a, Turn>;

    fn run(&mut self, mut data: Self::SystemData) {
        for _turn in (&mut data).join() {

        }
    }
}