use specs::prelude::*;

use crate::components::*;

use std::collections::VecDeque;

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
        WriteExpect<'a, VecDeque<Task>>
    );

    fn run(&mut self, mut data : Self::SystemData) {
        let taskqueue = &mut *data.5;

        loop {
            let task = taskqueue.front();
            if let Some(task) = task {
                // Find the first free AI and assign it the task
                let mut handled = false;
                for ai in (&mut data.0).join() {
                    if ai.task != Task::IDLE {
                        continue;
                    }
                    match task {
                        // TODO: task validation
                        Task::COLLECT(_target) => {
                            ai.task = *task;
                            taskqueue.pop_front();
                            handled = true;
                            break;
                        },
                        Task::STORE(_target) => {
                            ai.task = *task;
                            taskqueue.pop_front();
                            handled = true;
                            break;
                        },
                        Task::BUILD(_target) => {
                            ai.task = *task;
                            taskqueue.pop_front();
                            handled = true;
                            break;
                        }
                        Task::HOME(_target) => {
                            debug_assert!(false, "Home task should not be handled by scheduler right now...");
                        }
                        Task::IDLE => {
                            // do nothing...
                        }
                    }
                }
                // if we are unable to handle the task, we are somehow blocked...
                if !handled {
                    break;
                }
            }
            else {
                break;
            }
        }
    }
}