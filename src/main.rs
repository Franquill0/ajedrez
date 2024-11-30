mod pieces;
mod board;

use board::{Board, Position};
use pieces::{Piece, Color, EnPasant};

fn main(){
    let mut myBoard =  Board::newBoard();
    let pos1 = Position {x : 2, y : 2};
    let pos2 =  Position {x : 6, y : 8};
    let pos3 =  Position {x : 4, y : 1};
    let pawn = Piece::Pawn(Color::Black,EnPasant::Enable);

    myBoard.placePiece(pawn.clone(),&pos1);
    myBoard.placePiece(pawn.clone(),&pos2);
    myBoard.placePiece(pawn.clone(),&pos3);
    myBoard.display();
}
