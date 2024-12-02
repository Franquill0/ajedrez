use crate::pieces::{Square, Piece, Color, EnPasant};

pub struct Board {
    board: [[ Square ; 8] ; 8],
}
#[derive(Clone, Copy)]
pub struct Position {
    // x, y in [1,8]
    pub x: u8,
    pub y: u8,
}

impl Position {
    fn getYBoard(&self) -> usize {
        (8 - self.y) as usize
    }
    fn getXBoard(&self) -> usize{
        (self.x - 1) as usize
    }
}

impl Board {
    pub fn newBoard() -> Board {
        let board = Board {
            board : [[ Square::Empty ; 8] ; 8],
        };
        fulfillBoard(board)
    }
    pub fn canMove(&self, piece: &Piece, initialPos: &Position, finalPos: &Position) -> bool{
        true
    }
    pub fn placePiece(&mut self, piece: Piece, position: &Position){
        let x = position.getXBoard();
        let y = position.getYBoard();
        self.board[y][x] = Square::NonEmpty(piece);
    }
    fn removePiece(&mut self, position: &Position){
        let x = position.getXBoard();
        let y = position.getYBoard();
        self.board[y][x] = Square::Empty;
    }
    pub fn display(&self){
        for row in self.board.iter(){
            for square in row.iter() {
                let charPiece = 
                    match square {
                        Square::NonEmpty(piece) => piece.display(),
                        Square::Empty => String::from(" "),
                    };
                print!(" | {}",charPiece);
            }
            println!(" |");
        }
    }
}

fn fulfillBoard(board : Board) -> Board {
    board
}

