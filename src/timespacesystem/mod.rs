use specs::prelude::*;

use super::timespace::{Position, TimePosition, TimeVelocity, Velocity};

use super::userinput::{TimeUserInput, UserInput};

mod computation;
use self::computation::{get_new_pos, get_new_vel, time_horizon};

pub struct TimeSpaceSystem {
    speed_of_light: f32,
    earliest_horizon: usize,
}

impl TimeSpaceSystem {
    pub fn new(speed_of_light: f32, earliest_horizon: usize) -> Self {
        TimeSpaceSystem {
            speed_of_light: speed_of_light,
            earliest_horizon: earliest_horizon,
        }
    }

    fn set_earliest_horizon(&mut self, new_earliest_horizon: usize) {
        self.earliest_horizon = new_earliest_horizon;
    }
}

impl<'a> System<'a> for TimeSpaceSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TimeUserInput>,
        WriteStorage<'a, TimePosition>,
        WriteStorage<'a, TimeVelocity>,
    );

    fn run(&mut self, (ent, input, mut pos, mut vel): Self::SystemData) {
        let latest_horizon = (&input)
            .join()
            .map(|i| i.0.len())
            .max()
            .unwrap_or(usize::min_value());

        for t in self.earliest_horizon..=latest_horizon {
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
                .filter(|(e, _, _)| match input.get(*e) {
                    Some(input) => input.0.len() > t,
                    None => true,
                })
                .for_each(|(ent, pos, vel)| {
                    let new_vel = get_new_vel(&vel.0[t], input.get(ent), t);
                    let new_pos = get_new_pos(&pos.0[t], &new_vel);

                    vel.0.push(new_vel);
                    pos.0.push(new_pos);
                });
        }

        let new_earliest_horizon = (&pos, &input)
            .join()
            .map(|(p, _)| p.0.len())
            .min()
            .unwrap_or(usize::max_value());
        self.set_earliest_horizon(new_earliest_horizon);
    }
}
