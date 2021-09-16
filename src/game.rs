use crate::ant::{Ant, Direction};
use std::fmt;

pub struct Game {
    board: Vec<Vec<bool>>,
    ant: Ant,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            board: vec![vec![false; width]; height],
            ant: Ant::new(2, 17, Direction::DOWN),
        }
    }

    pub fn play(&mut self) {
        let position = self.ant.get_position();
        let active = self.board[position.0][position.1];

        if active { 
            self.ant.turn(Direction::LEFT);
        } else { 
            self.ant.turn(Direction::RIGHT);
        }
        self.ant.make_move(1);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = String::new();
        let position = self.ant.get_position();

        board = board + "\n";
        for (row_n, row) in self.board.iter().enumerate() {
            for (place_n, place) in row.iter().enumerate() {
                if position.0 == place_n && position.1 == row_n {
                    board = board + self.ant.draw();
                } else {
                    let cell = if *place { "◼️ "} else { " " };
                    board = board + cell;
                }
            }
            board = board + "\n";
        }
        board = board + "\n";

        write!(f, "{}", board)
    }
}