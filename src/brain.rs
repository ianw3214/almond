use specs::prelude::*;
// use rand::prelude::*;

use crate::components::*;

const RESOURCE_DISTANCE_THRESHOLD : i32 = 10;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (
        WriteStorage<'a, Brain>,
        WriteStorage<'a, ResourceSource>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Movement>,
        // List of all entities
        Entities<'a>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        // let mut rng = thread_rng();

        // record moves to make at the end
        let mut moves : Vec<(Entity, Entity)> = Vec::new();
        let mut collects : Vec<(Entity, Entity)> = Vec::new();

        // Let things with  brains try to collect resources
        for (entity, brain, pos) in (&data.5, &mut data.0, &data.3).join() {
            match brain.curr_target {
                Some(target) => {
                    // try to move to target and collect it
                    let target_pos = data.3.get(target).unwrap();
                    let dist = (pos.x - target_pos.x).abs() + (pos.y - target_pos.y).abs();
                    if dist > RESOURCE_DISTANCE_THRESHOLD {
                        // move to the resource
                        moves.push((entity, target));
                    }
                    else {
                        // collect the resource
                        collects.push((entity, target));
                    }
                },
                None => {
                    // try to find a target
                    for (entity, source) in (&data.5, &mut data.1).join() {
                        if source.amount > 0 {
                            brain.curr_target = Some(entity);
                        }
                    }
                }
            }
        }

        // move entitites to targets
        for pairs in moves {
            let to = data.3.get(pairs.1).unwrap();
            let target_x = to.x;
            let target_y = to.y;
            // let mut from = data.3.get_mut(pairs.0).unwrap();
            // from.x = target_x;
            // from.y = target_y;
            let move_data = data.4.get_mut(pairs.0).unwrap();
            move_data.target = Some((target_x, target_y));
        }

        // collect resource targets
        let mut removes : Vec<Entity> = Vec::new();
        for pairs in collects {
            let inv = data.2.get_mut(pairs.0).unwrap();
            let mut src = data.1.get_mut(pairs.1).unwrap();
            for resources in &mut inv.resources {
                if resources.0 == src.resource_type {
                    resources.1 = resources.1 + 1;
                    src.amount = src.amount - 1;
                    if src.amount <= 0 {
                        // remove the resource if depleted
                        removes.push(pairs.1);
                        let mut brain = data.0.get_mut(pairs.0).unwrap();
                        brain.curr_target = None;
                    }
                }
            }
        }

        // remove depleted resources
        for entity in removes {
            data.5.delete(entity).expect("");
        }

    }
}