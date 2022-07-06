use specs::prelude::*;

use crate::components::*;

use crate::GridSize;

pub struct Grid;

impl<'a> System<'a> for Grid {
    type SystemData = (
        ReadExpect<'a, GridSize>,
        ReadStorage<'a, GridPosition>,
        WriteStorage<'a, WorldPosition>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let grid_size = data.0;

        for (grid_pos, world_pos) in (&data.1, &mut data.2).join() {
            world_pos.0.x = grid_pos.x * grid_size.width;
            world_pos.0.y = grid_pos.y * grid_size.height;
        }
    }
}