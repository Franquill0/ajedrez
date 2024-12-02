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

impl Square {
    pub fn getPiece(self) -> Option<Piece> {
        match self {
            Square::Empty => None,
            Square::NonEmpty(piece) => Some(piece),
        }
    }
}

impl Piece {
    pub fn display(&self) -> String {
        let piece = match self {
            Piece::King(_)   =>"k",
            Piece::Queen(_)  =>"q",
            Piece::Knight(_) =>"n",
            Piece::Bishop(_) =>"b",
            Piece::Pawn(_,_) =>"p",
            Piece::Rook(_)   =>"r",
        };
        match self.getColor() {
            Color::White => piece.to_uppercase(),
            _ => piece.to_string(),
        }
    }
    pub fn getColor(self) -> Color {
        match self {
            Piece::King(color) | Piece::Queen(color) | Piece::Knight(color) | Piece::Bishop(color) | Piece::Rook(color) => color,
            Piece::Pawn(color,_) => color,
        }
    }
}
