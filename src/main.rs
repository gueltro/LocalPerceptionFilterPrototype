extern crate specs;
use specs::prelude::*;

mod timespace;
use timespace::{Position, TimePosition, TimeVelocity, Velocity};

mod userinput;
use userinput::{TimeUserInput, UserInput};

mod timespacesystem;
use timespacesystem::TimeSpaceSystem;

fn main() {
    let mut world = World::new();

    //Register Components
    world.register::<TimePosition>();
    world.register::<TimeVelocity>();
    world.register::<TimeUserInput>();

    //Create Initial Entities
    world
        .create_entity()
        .with(TimePosition(vec![Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }]))
        .with(TimeVelocity(vec![Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }]))
        .build();
    world
        .create_entity()
        .with(TimeUserInput(vec![None]))
        .with(TimePosition(vec![Position {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }]))
        .with(TimeVelocity(vec![Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }]))
        .build();

    //Dispatch TimeSpaceSystem
    let mut dispatcher = DispatcherBuilder::new()
        .with(TimeSpaceSystem::new(100.0, 0), "TimeSpaceSystem", &[])
        .build();
    dispatcher.setup(&mut world.res);

    loop {
        dispatcher.dispatch(&mut world.res);
    }
}
