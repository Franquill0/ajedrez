#[derive(Clone, Copy)]
pub enum Square {
    Empty,
    NonEmpty(Piece),
}
#[derive(Clone, Copy)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Pawn(Color,EnPasant),
}
#[derive(Clone, Copy)]
pub enum EnPasant {
    Enable,
    Disable,
}

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White,
}


impl Piece {
    pub fn display(&self) -> &str {
        match self {
            Piece::King(color)   => if let Color::Black = color {"k"} else {"K"},
            Piece::Queen(color)  => if let Color::Black = color {"q"} else {"Q"},
            Piece::Knight(color) => if let Color::Black = color {"n"} else {"N"},
            Piece::Bishop(color) => if let Color::Black = color {"b"} else {"B"},
            Piece::Pawn(color,_) => if let Color::Black = color {"p"} else {"P"},
            Piece::Rook(color)   => if let Color::Black = color {"r"} else {"R"},
        }
    }
}
