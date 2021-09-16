mod ant;
mod game;

use std::{thread, time::Duration};

const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const SLEEP: Duration = Duration::from_millis(1000);

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT);

    loop {
        println!("{}", game);
        game.play();        
        thread::sleep(SLEEP);
    }
}
