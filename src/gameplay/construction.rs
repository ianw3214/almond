use specs::prelude::*;

use std::cmp;

use crate::{components::*, GameRequests};

pub struct ConstructionSystem;

impl<'a> System<'a> for ConstructionSystem {
    type SystemData = (
        WriteStorage<'a, ResourceStorage>,
        // global resources
        WriteExpect<'a, Vec<GameRequests>>,
        WriteExpect<'a, Vec<Task>>,
        // List of all entities
        Entities<'a>,
        Read<'a, LazyUpdate>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        // TODO: Handle all requests instead of just one
        let game_requests = &mut *data.1;
        let request = game_requests.pop();
        if let Some(GameRequests::ConstructionStart(x, y)) = request {
            // check if there are enough resources to build a building
            // TODO: Cache resources to avoid recalculations?
            let mut wood_count = 0;
            let mut flint_count = 0;
            for storage in (&data.0).join() {
                wood_count += storage.resources[0].1;
                flint_count += storage.resources[1].1;
            }
            if wood_count >= 5 && flint_count >= 5 {
                wood_count = 5;
                flint_count = 5;
                for storage in (&mut data.0).join() {
                    let wood_take = cmp::min(storage.resources[0].1, wood_count);
                    let flint_take = cmp::min( storage.resources[1].1, flint_count);
                    storage.resources[0].1 -= wood_take;
                    storage.resources[1].1 -= flint_take;
                }
                // consume the resources and build the building
                let building = data.3.create();
                data.4.insert(building, Position{ x : x, y : y });
                data.4.insert(building, Renderable{ i : 6 });
                data.4.insert(building, Construction{ timer : 10.0 });
                data.4.insert(building, BoundingBox{ width : 40, height : 40, x_offset : 0, y_offset : 0 });
                data.4.insert(building, Housing{ capacity : 2, num_tenants : 0 });
                // Add a task to construct the building
                let taskqueue = &mut *data.2;
                taskqueue.push(Task::BUILD(building));
            }
        }
    }
}