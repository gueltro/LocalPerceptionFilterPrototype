use specs::prelude::*;

pub enum UserInput {
    Up,
    Down,
    Left,
    Right,
}

pub struct TimeUserInput(pub Vec<Option<UserInput>>);

impl Component for TimeUserInput {
    type Storage = VecStorage<Self>;
}
