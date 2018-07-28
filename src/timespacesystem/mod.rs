use specs::prelude::*;

use super::timespace::{Position, TimePosition, TimeVelocity, Velocity};

use super::userinput::{TimeUserInput, UserInput};

mod computation;
use self::computation::{get_new_pos, get_new_vel, time_horizon};

pub struct TimeSpaceSystem {
    pub speed_of_light: f32,
}

impl<'a> System<'a> for TimeSpaceSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TimeUserInput>,
        WriteStorage<'a, TimePosition>,
        WriteStorage<'a, TimeVelocity>,
    );

    fn run(&mut self, (ent, input, mut pos, mut vel): Self::SystemData) {
        for t in 0..1 {
            let latest_user_states: Vec<(usize, Position)> = (&pos, &input)
                .join()
                .map(|(p, i)| (p.0.len(), p.0[p.0.len() - 1].clone()))
                .collect();

            (&*ent, &mut pos, &mut vel)
                .join()
                .filter(|(_, p, _)| p.0.len() == t + 1)
                .filter(|(_, p, _)| {
                    time_horizon(&p.0[t], &latest_user_states, self.speed_of_light) >= t
                })
                .filter(|(e,_,_)|
                    match input.get(*e){
                        Some(input) => input.0.len() > t,
                        None => true
                    }
                )
                .for_each(|(ent, pos, vel)| {
                    let new_vel = get_new_vel(&vel.0[t], input.get(ent), t);
                    let new_pos = get_new_pos(&pos.0[t], &new_vel);

                    vel.0.push(new_vel);
                    pos.0.push(new_pos);
                });
        }
    }
}
