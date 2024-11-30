

pub enum Square {
    Empty,
    NonEmpty(Piece),
}
pub enum Piece {
    King(Color),
    Queen(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Pawn(Color,EnPasant),
}
pub enum EnPasant {
    Enable,
    Disable,
}

pub enum Color {
    Black,
    White,
}

