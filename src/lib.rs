use std::collections::VecDeque;

use sdl2::rect::{Point, Rect};

pub const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { Up, Down, Left, Right }

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
