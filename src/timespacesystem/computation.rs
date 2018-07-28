use specs::prelude::*;

use super::super::timespace::{Position, TimePosition, TimeVelocity, Velocity};

use super::super::userinput::{TimeUserInput, UserInput};

pub fn get_new_vel(vel: &Velocity, input: Option<&TimeUserInput>, t: usize) -> Velocity {
    let mut new_vel = vel.clone();

    if let Some(input) = input {
        if input.0.len() >= t {
            match &input.0[t] {
                Some(UserInput::Up) => {
                    new_vel.y += 1.0;
                }
                Some(UserInput::Down) => {
                    new_vel.y -= 1.0;
                }
                Some(UserInput::Left) => {
                    new_vel.x += 1.0;
                }
                Some(UserInput::Right) => {
                    new_vel.x -= 1.0;
                }
                None => {}
            }
        }
    }

    new_vel
}

pub fn get_new_pos(pos: &Position, vel: &Velocity) -> Position {
    Position {
        x: pos.x + vel.x,
        y: pos.y + vel.y,
        z: pos.x + vel.y,
    }
}
