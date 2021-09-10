#[derive(Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Ant {
    x: u8,
    y: u8,
    direction: Direction,   
}

impl Ant {
    pub fn new(x: u8, y: u8, direction: Direction) -> Ant {
        Ant { x, y, direction }
    }

    pub fn make_move(&mut self, amount: u8) -> (u8, u8) {
        match self.direction {
            Direction::UP => self.y.wrapping_add(amount),
            Direction::DOWN => self.y.wrapping_sub(amount),
            Direction::LEFT => self.x.wrapping_sub(amount),
            Direction::RIGHT => self.x.wrapping_add(amount),
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
    
    pub fn get_position(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}