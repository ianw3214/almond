use specs::prelude::*;

use crate::components::*;

use rand::prelude::*;

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
        WriteStorage<'a, Construction>,
        // List of all entities
        Entities<'a>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        // record moves to make at the end
        let mut collects : Vec<(Entity, Entity)> = Vec::new();
        let mut stores : Vec<(Entity, Entity)> = Vec::new();
        let mut constructs : Vec<(Entity, Entity)> = Vec::new();

        // Let things with  brains try to collect resources
        for (entity, brain, pos, movement) in (&data.7, &mut data.0, &data.4, &mut data.5).join() {
            match brain.task {
                Task::COLLECT(target) => {
                    // try to move to target and handle it
                    let target_pos = data.4.get(target).unwrap();
                    let dist = (pos.x - target_pos.x).abs() + (pos.y - target_pos.y).abs();
                    if dist > DISTANCE_THRESHOLD {
                        // move to the resource
                        movement.target = Some((target_pos.x, target_pos.y));
                    }
                    else {
                        collects.push((entity, target));
                    }
                },
                Task::STORE(target) => {
                    // try to move to target and handle it
                    let target_pos = data.4.get(target).unwrap();
                    let dist = (pos.x - target_pos.x).abs() + (pos.y - target_pos.y).abs();
                    if dist > DISTANCE_THRESHOLD {
                        // move to the resource
                        movement.target = Some((target_pos.x, target_pos.y));
                    }
                    else {
                        stores.push((entity, target));
                    }
                },
                Task::BUILD(target) => {
                    // try to move to target and handle it
                    let target_pos = data.4.get(target).unwrap();
                    let dist = (pos.x - target_pos.x).abs() + (pos.y - target_pos.y).abs();
                    if dist > DISTANCE_THRESHOLD {
                        // move to the resource
                        movement.target = Some((target_pos.x, target_pos.y));
                    }
                    else {
                        constructs.push((entity, target));
                    }
                }
                Task::IDLE => {
                    // give the brain a 1 in 1000 chance to randomly move
                    brain.task = Task::IDLE;
                    let mut rng = thread_rng();
                    let index : i32 = rng.gen_range(0..400);
                    if index == 0 {
                        let x_offset = rng.gen_range(0..30) - 15;
                        let y_offset = rng.gen_range(0..30) - 15;
                        movement.target = Some((pos.x + x_offset, pos.y + y_offset));
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
                        brain.task = Task::IDLE;
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
                brain.task = Task::IDLE;
            }
        }

        // construction targets
        for pairs in constructs {
            let construction = data.6.get_mut(pairs.1).unwrap();
            construction.counter = construction.counter + 1;
            if construction.counter > 100 {
                let mut brain = data.0.get_mut(pairs.0).unwrap();
                brain.task = Task::IDLE;
                data.6.remove(pairs.1);
            }
        }

        // remove depleted resources
        for entity in removes {
            data.7.delete(entity).expect("");
        }

    }
}