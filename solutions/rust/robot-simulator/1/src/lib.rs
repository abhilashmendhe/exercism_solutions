// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (i32, i32),
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            pos: (x, y),
            direction: d
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let dir: Direction = match self.direction {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };
        Self {
            pos: self.pos,
            direction: dir
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let dir: Direction = match self.direction {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        };
        Self {
            pos: self.pos,
            direction: dir
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let newpos: (i32, i32) = match self.direction {
            Direction::East => (self.pos.0 + 1, self.pos.1),
            Direction::North => (self.pos.0, self.pos.1 + 1),
            Direction::South => (self.pos.0, self.pos.1 - 1),
            Direction::West => (self.pos.0 - 1, self.pos.1)
        };
        Self {
            pos: newpos,
            direction: self.direction
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut s = self;
        for ch in instructions.chars() {
            s = match ch {
                'R' => s.turn_right(),
                'L' => s.turn_left(),
                'A' => s.advance(),
                _ => s,
            };
        }
        s
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
