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

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Piece {
    pub fn display_full_name(&self) -> String {
        let piece = match self {
            Piece::King(_)   =>"King",
            Piece::Queen(_)  =>"Queen",
            Piece::Knight(_) =>"Knight",
            Piece::Bishop(_) =>"Bishop",
            Piece::Pawn(_)   =>"Pawn",
            Piece::Rook(_)   =>"Rook",
        };
        piece.to_string()
    }
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
    pub fn are_same_color(&self, other: &Self) -> bool {
        self.get_color() == other.get_color()
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
             _  => Err(format!("'{}' not a piece!",piece)),
        }
    }
    pub fn is_king(&self) -> bool {
        if let Piece::King(_) = *self {
            true
        } else {
            false
        }
    }
    pub fn is_queen_or_rook_of_color(&self, color: Color) -> bool {
        self.is_queen_of_color(color) || self.is_rook_of_color(color)
    }
    pub fn is_queen_or_bishop_of_color(&self, color: Color) -> bool {
        self.is_queen_of_color(color) || self.is_bishop_of_color(color)
    }
    pub fn is_knight_of_color(&self, color: Color) -> bool {
        if let Self::Knight(knight_color) = *self {
            knight_color == color
        } else {
            false
        }
    }
    fn is_bishop_of_color(&self, color: Color) -> bool {
        if let Self::Bishop(bishop_color) = *self {
            bishop_color == color
        } else {
            false
        }
    }
    fn is_queen_of_color(&self, color:Color) -> bool {
        if let Self::Queen(queen_color) = *self {
            queen_color == color
        } else {
            false
        }
    }
    fn is_rook_of_color(&self, color:Color) -> bool {
        if let Self::Rook(queen_color) = *self {
            queen_color == color
        } else {
            false
        }
    } 

    pub fn is_pawn_of_color(&self, color: Color) -> bool {
        if let Self::Pawn(pawn_color) = *self {
            pawn_color == color
        } else {
            false
        }

    }

}

