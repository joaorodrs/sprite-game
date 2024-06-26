use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return, // no change
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = PLAYER_MOVEMENT_SPEED;
                    vel.direction.push_back(direction);
                },
                MovementCommand::Stop(direction) => {
                    vel.direction.retain(|s| s != direction);

                    if vel.direction.len() != 0 {
                        return
                    }

                    vel.speed = 0;
                },
            }
        }
    }
}
