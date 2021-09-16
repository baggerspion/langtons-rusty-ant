use crate::ant::{Ant, Direction};

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

    pub fn advance(&mut self) {
        let position = self.ant.get_position();
        let active = self.board[position.0][position.1];

        // Turn left or right depending on the state 
        // of the current square
        if active { 
            self.ant.turn(Direction::LEFT);
        } else { 
            self.ant.turn(Direction::RIGHT);
        }

        // Flip the state of the square
        let position = self.ant.get_position();
        self.board[position.0][position.1] = !self.board[position.0][position.1];

        // Move one square forward
        self.ant.make_move(1);
    }

    pub fn draw(&self) -> String {
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

        board
    }
}