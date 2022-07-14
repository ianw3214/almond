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
        let grid_size = &*data.0;

        let half_width : i32 = grid_size.width / 2;
        let three_quarter_height : i32 = grid_size.height * 3 / 4;

        for (grid_pos, world_pos) in (&data.1, &mut data.2).join() {
            world_pos.point.x = grid_pos.x * grid_size.width + half_width;
            world_pos.point.y = grid_pos.y * grid_size.height + three_quarter_height;
        }
    }
}