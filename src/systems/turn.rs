use specs::prelude::*;

use crate::components::*;

pub struct TurnSystem {
    pub last_prio : i32
}

impl<'a> System<'a> for TurnSystem {
    type SystemData = WriteStorage<'a, Turn>;

    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: There must be some way to do this without looping twice
        //  - or maybe i am stupid
        let mut next_biggest_prio = std::i32::MAX;
        let mut smallest_prio = std::i32::MIN;
        let mut use_next = false;
        for turn in (&data).join() {
            if turn.priority < smallest_prio || smallest_prio == std::i32::MIN {
                smallest_prio = turn.priority;
            }
            if turn.priority < next_biggest_prio && turn.priority > self.last_prio {
                next_biggest_prio = turn.priority;
            }
            // TODO: Handle overlapping priorities 
            // assume priorities are unique for now
            if !turn.current && turn.priority == self.last_prio {
                use_next = true;
            }
        }
        if use_next || self.last_prio == std::i32::MIN {
            let target = if next_biggest_prio == std::i32::MAX { smallest_prio } else { next_biggest_prio };
            for turn in (&mut data).join() {
                if turn.priority == target {
                    self.last_prio = target;
                    turn.current = true;
                    break;
                }
            }
        }
    }
}