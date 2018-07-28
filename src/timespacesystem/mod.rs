use specs::prelude::*;

use super::timespace::{Position, TimePosition, TimeVelocity, Velocity};

use super::userinput::{TimeUserInput, UserInput};

mod computation;
use self::computation::{get_new_pos, get_new_vel};

pub struct TimeSpaceSystem;

impl<'a> System<'a> for TimeSpaceSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TimeUserInput>,
        WriteStorage<'a, TimePosition>,
        WriteStorage<'a, TimeVelocity>,
    );

    fn run(&mut self, (ent, input, mut pos, mut vel): Self::SystemData) {
        for t in 0..1 {
            for (ent, pos, vel) in (&*ent, &mut pos, &mut vel).join() {
                let new_vel = get_new_vel(&vel.0[t], input.get(ent), t);
                let new_pos = get_new_pos(&pos.0[t], &new_vel);

                vel.0.push(new_vel);
                pos.0.push(new_pos);
            }
        }
    }
}
