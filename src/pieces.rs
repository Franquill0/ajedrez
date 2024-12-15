use crate::board::{Board, Position};

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
    Pawn(Color),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Square {
    pub fn get_piece(self) -> Option<Piece> {
        match self {
            Square::Empty => None,
            Square::NonEmpty(piece) => Some(piece),
        }
    }
    pub fn display(&self) -> String {
        match self {
            Square::NonEmpty(piece) => piece.display(),
            Square::Empty => String::from(" "),
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
            Piece::Pawn(_)   =>"p",
            Piece::Rook(_)   =>"r",
        };
        match self.get_color(){
            Color::White => piece.to_uppercase(),
            _ => piece.to_string(),
        }
    }
    pub fn get_color(&self) -> Color {
        match self {
            Piece::King(color)   |
            Piece::Queen(color)  |
            Piece::Knight(color) |
            Piece::Bishop(color) |
            Piece::Rook(color)   |
            Piece::Pawn(color) => *color,
        }
    }
    pub fn piece_from_char(char_piece: char) -> Result<Piece, String> {
        let color =
            if char_piece.is_uppercase() {
                Color::White
            } else {
                Color::Black
        };
        let piece = char_piece.to_lowercase().next().unwrap();
        match piece {
            'k' => Ok(Piece::King(color)),
            'q' => Ok(Piece::Queen(color)),
            'b' => Ok(Piece::Bishop(color)),
            'n' => Ok(Piece::Knight(color)),
            'r' => Ok(Piece::Rook(color)),
            'p' => Ok(Piece::Pawn(color)),
            _ => Err(format!("'{}' not a piece!",piece)),
        }
    }
    pub fn is_king(piece: &Piece, color: &Color) -> bool {
        if let Piece::King(c) = *piece {
            if *color == c {
                return true;
            }
        }
        false
    }

}

