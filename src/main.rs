mod ant;
mod game;

extern crate ncurses;

use ncurses::*;
use std::{thread, time::Duration};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const SLEEP: Duration = Duration::from_millis(1000);

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT);

    initscr();
    noecho();

    loop {
        addstr(&game.draw());
        refresh();
        game.advance();        
        thread::sleep(SLEEP);
        clear();
    }
}
