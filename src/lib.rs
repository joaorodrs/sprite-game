use std::collections::VecDeque;

use sdl2::rect::{Point, Rect};

use specs::prelude::*;
use specs_derive::Component;

pub const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { Up, Down, Left, Right }

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: VecDeque<Direction>,
    pub last_direction: Option<Direction>,
    pub current_frame: i32,
}

impl Player {
    /// # Unmoves the player.
    ///
    /// The direction is the Direction that needs to be removed from player current directions.
    pub fn unmove_player(&mut self, direction: Direction) {
        let &last_direction = self.direction.back().unwrap();

        if let Some(index) = self.direction.iter().position(|&x| x == direction) {
            self.direction.remove(index);
        }

        if self.direction.len() != 0 {
            self.speed = PLAYER_MOVEMENT_SPEED;
        } else {
            self.speed = 0;
            self.last_direction = Some(last_direction);
        }
    }

    /// # Moves the player.
    ///
    /// The direction is the Direction that will be added to the player's Directions set.
    pub fn move_player(&mut self, direction: Direction, opposite: Direction) {
        if self.direction.contains(&opposite) {
            self.speed = 0;
        } else {
            self.speed = PLAYER_MOVEMENT_SPEED;
        }
        if !self.direction.contains(&direction) {
            self.direction.push_back(direction);
        };
    }

    /// # Creates a new Player.
    ///
    /// Creates a new player from Player struct.
    pub fn new() -> Player {
        let position = Point::new(0, 0);
        let sprite = Rect::new(0, 0, 26, 36);

        Player {
            position,
            sprite,
            speed: 0,
            direction: VecDeque::from([]),
            current_frame: 0,
            last_direction: None,
        }
    }

}
