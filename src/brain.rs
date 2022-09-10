use specs::prelude::*;

use crate::components::*;

const DISTANCE_THRESHOLD : i32 = 10;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (
        WriteStorage<'a, Brain>,
        WriteStorage<'a, ResourceSource>,
        WriteStorage<'a, ResourceStorage>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Movement>,
        // List of all entities
        Entities<'a>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        // record moves to make at the end
        let mut collects : Vec<(Entity, Entity)> = Vec::new();
        let mut stores : Vec<(Entity, Entity)> = Vec::new();

        // Let things with  brains try to collect resources
        for (entity, brain, pos, movement) in (&data.6, &mut data.0, &data.4, &mut data.5).join() {
            match brain.curr_target {
                Some(target) => {
                    // try to move to target and handle it
                    let target_pos = data.4.get(target).unwrap();
                    let dist = (pos.x - target_pos.x).abs() + (pos.y - target_pos.y).abs();
                    if dist > DISTANCE_THRESHOLD {
                        // move to the resource
                        movement.target = Some((target_pos.x, target_pos.y));
                    }
                    else {
                        match brain.task {
                            TaskType::COLLECT => {
                                // collect the resource
                                collects.push((entity, target));
                            },
                            TaskType::STORE => {
                                // store the resource
                                stores.push((entity, target));
                            }
                        }
                    }
                },
                None => {
                    // try to find a target
                    for (entity, source) in (&data.6, &mut data.1).join() {
                        if source.amount > 0 {
                            brain.curr_target = Some(entity);
                            brain.task = TaskType::COLLECT;
                        }
                    }
                    if let None = brain.curr_target {
                        let inventory = data.3.get_mut(entity).unwrap();
                        for resource in &inventory.resources {
                            if resource.1 > 0 {
                                // try to find a target if there are resources to store
                                for (entity, _storage) in (&data.6, &mut data.2).join() {
                                    // TODO: should also check that storage isn't full
                                    brain.curr_target = Some(entity);
                                    brain.task = TaskType::STORE;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        // collect resource targets
        let mut removes : Vec<Entity> = Vec::new();
        for pairs in collects {
            let inv = data.3.get_mut(pairs.0).unwrap();
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

        // store resource targets
        for pairs in stores {
            let inv = data.3.get_mut(pairs.0).unwrap();
            let storage = data.2.get_mut(pairs.1).unwrap();
            let mut handled = false;
            for resources in &mut inv.resources {
                if resources.1 <= 0 {
                    continue;
                }
                handled = true;
                for storage_resource in &mut storage.resources {
                    if resources.0 == storage_resource.0 {
                        resources.1 = resources.1 - 1;
                        storage_resource.1 = storage_resource.1 + 1;
                    }
                }
            }
            if !handled {
                let mut brain = data.0.get_mut(pairs.0).unwrap();
                brain.curr_target = None;
            }
        }

        // remove depleted resources
        for entity in removes {
            data.6.delete(entity).expect("");
        }

    }
}