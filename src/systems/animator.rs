use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Sprite>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for (anim, sprite) in (&mut data.0, &mut data.1).join() {
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