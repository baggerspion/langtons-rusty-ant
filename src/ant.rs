use crate::{HEIGHT, WIDTH};

#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Ant {
    x: usize,
    y: usize,
    direction: Direction,   
}

impl Ant {
    pub fn new(x: usize, y: usize, direction: Direction) -> Ant {
        Ant { x, y, direction }
    }

    pub fn make_move(&mut self, amount: usize) -> (usize, usize) {
        let current_direction = self.direction;
        let mut current_position = self.get_position();

        match self.direction {
            Direction::UP =>
                current_position.1 = (current_position.1 + amount) % HEIGHT,
            Direction::DOWN =>
                current_position.1 = ((current_position.1 - amount) + HEIGHT) % HEIGHT,
            Direction::LEFT =>
                current_position.0 = ((current_position.0 - amount) + WIDTH) % WIDTH,
            Direction::RIGHT =>
                current_position.0 = (current_position.0 + amount) % WIDTH,
        };

        *self = Self {
            x: current_position.0,
            y: current_position.1,
            direction: current_direction,
        };

        (self.x, self.y)
    }

    pub fn turn(&mut self, direction: Direction) {
        self.direction = match direction {
            Direction::LEFT => {
                match self.direction {
                    Direction::UP => Direction::LEFT,
                    Direction::RIGHT => Direction::UP,
                    Direction::DOWN => Direction::RIGHT,
                    Direction::LEFT => Direction::DOWN,
                }
            },
            Direction::RIGHT => {
                match self.direction {
                    Direction::UP => Direction::RIGHT,
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                }
            },
            _ => self.direction,
        };
    }
    
    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn draw(&self) -> &str {
        match self.direction {
            Direction::UP => "▲",
            Direction::RIGHT => "▶",
            Direction::DOWN => "▼",
            Direction::LEFT => "◀",
        }
    }
}