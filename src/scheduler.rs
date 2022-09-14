use specs::prelude::*;

use crate::components::*;

pub struct Scheduler;

impl<'a> System<'a> for Scheduler {
    type SystemData = (
        WriteStorage<'a, Brain>,
        WriteStorage<'a, ResourceSource>,
        WriteStorage<'a, ResourceStorage>,
        WriteStorage<'a, Inventory>,
        // List of all entities
        Entities<'a>,
        // Global resources
        WriteExpect<'a, Vec<Task>>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        let taskqueue = &mut *data.5;

        // TODO: Queue implementation?
        let task = taskqueue.last();
        if let Some(task) = task {
            // Find the first free AI and assign it the task
            for ai in (&mut data.0).join() {
                if ai.task != Task::IDLE {
                    continue;
                }
                match task {
                    // TODO: task validation
                    Task::COLLECT(_target) => {
                        ai.task = *task;
                        taskqueue.pop();
                        break;
                    },
                    Task::STORE(_target) => {
                        ai.task = *task;
                        taskqueue.pop();
                        break;
                    },
                    Task::BUILD(_target) => {
                        ai.task = *task;
                        taskqueue.pop();
                        break;
                    }
                    Task::IDLE => {
                        // do nothing...
                    }
                }
            }
        }
        
    }
}