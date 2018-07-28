use specs::prelude::*;

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct TimeVelocity(pub Vec<Velocity>);

pub struct TimePosition(pub Vec<Position>);

impl Component for TimeVelocity {
    type Storage = VecStorage<Self>;
}

impl Component for TimePosition {
    type Storage = VecStorage<Self>;
}
