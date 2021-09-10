pub struct Board {
    pub squares: [[bool; 255]; 255]
}

impl Board {
    pub fn new() -> Board {
        Board { squares: [[false; 255]; 255] }
    }

    pub fn flip(&mut self, x: u8, y:u8) -> bool {
        self.squares[x as usize][y as usize] = !self.squares[x as usize][y as usize];
        self.squares[x as usize][y as usize]
    }

    pub fn get_square(&self, x: u8, y: u8) -> bool {
        self.squares[x as usize][y as usize]
    }
}