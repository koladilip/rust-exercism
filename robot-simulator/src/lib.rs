// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

use Direction::*;

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Robot {
            d: new_direction,
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction = match self.d {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Robot {
            d: new_direction,
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            North => Self {
                y: self.y + 1,
                ..self
            },

            East => Self {
                x: self.x + 1,
                ..self
            },

            South => Self {
                y: self.y - 1,
                ..self
            },

            West => Self {
                x: self.x - 1,
                ..self
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => panic!("Invalid instruction"),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
