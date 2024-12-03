mod pieces;
mod board;

use board::{Board};

fn main(){
    let mut my_board = Board::new_board();
    my_board.initial_position();
    my_board.display();
}
