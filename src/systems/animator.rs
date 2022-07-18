use specs::prelude::*;

use crate::components::*;

use std::time::{Instant, Duration};

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Sprite>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for (anim, sprite) in (&mut data.0, &mut data.1).join() {
            // hard coded at 10 fps currently
            //  - This should be set per animation in the future
            let update_duration = Duration::from_millis(100);
            if anim.last_update.elapsed() < update_duration {
                continue
            }
            anim.last_update = Instant::now();
            anim.current_frame = anim.current_frame + 1;
            if anim.current_frame > anim.animations[anim.current_anim].1 {
                anim.current_frame = anim.animations[anim.current_anim].0;
            }
            // TODO: Make this not hard coded
            // Assume width and height stay the same for now...
            sprite.region.x = (anim.current_frame as i32 % 4) * 30;
            sprite.region.y = (anim.current_frame as i32 / 4) * 40;
        }
    }
}