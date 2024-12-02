mod pieces;
mod board;

use board::{Board, Position};
use pieces::{Piece, Color};

fn main(){
    let mut my_board = Board::new_board();
    my_board.initial_position();

    my_board.display();
}
