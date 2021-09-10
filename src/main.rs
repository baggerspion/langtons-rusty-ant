mod ant;
mod board;

use ant::{Ant, Direction};
use board::Board;

fn main() {
    let mut ant = Ant::new(0, 0, Direction::UP);
    let mut board = Board::new();

    for _ in 1..200 {        
        // We turn left or right depending on the square state
        let position = ant.get_position();
        let square = board.get_square(position.0, position.1);

        if square {
            ant.turn(Direction::LEFT);
        } else {
            ant.turn(Direction::RIGHT);
        }

        // No matter what we flip the state of the current square and then move on
        board.flip(position.0, position.1);
        ant.make_move(1);
    }
}
