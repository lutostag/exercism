use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(PartialEq, Debug, FromPrimitive, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        FromPrimitive::from_i32((*self as i32 + 90) % 360).unwrap()
    }
    pub fn turn_left(&self) -> Self {
        FromPrimitive::from_i32((*self as i32 - 90 + 360) % 360).unwrap()
    }
}

#[derive(Debug)]
pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = self.direction.turn_right();
        Robot { direction, ..self }
    }

    pub fn turn_left(self) -> Self {
        let direction = self.direction.turn_left();
        Robot { direction, ..self }
    }

    pub fn advance(self) -> Self {
        let mut position = self.position;
        match self.direction {
            Direction::North => position.1 += 1,
            Direction::East => position.0 += 1,
            Direction::South => position.1 -= 1,
            Direction::West => position.0 -= 1,
        };
        Robot { position, ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
