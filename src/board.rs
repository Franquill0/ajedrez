mod pieces;
use pieces::{Square, Piece};

pub struct Board {
    board: [[ Square::Empty ; 8] ; 8]
}

struct Position {
    // x, y in [1,8]
    x: u8,
    y: u8,
}

impl Position {
    fn getYBoard(&self) -> u8 {
         8 - self.y
    }
    fn getXBoard(&self){
        self.x - 1
    }
}

impl Board {
    pub fn canMove(&self, piece: &Piece, initialPos: &Position, finalPos: &Position) -> bool{
        true
    }
    fn placePiece(&mut self, piece: Piece, position: &Position){
        let x = position.getXBoard();
        let y = position.getYBoard();
        self.board[y][x] = Square::NonEmpty(piece);
    }
    fn removePiece(&mut self, position: &Position){
        let x = position.getXBoard();
        let y = position.getYBoard();
        self.board[y][x] = Square::Empty;
    }
}
