use crate::pieces::{Square, Piece, Color};

pub struct Board {
    board: [[ Square ; 8] ; 8],
}
#[derive(Clone, Copy)]
pub struct Position {
    // x, y in [1,8]
    x: u8,
    y: u8,
}

impl Position {
    pub fn new_position(x: u8, y: u8) -> Self {
        assert!( 0 < x && x < 9, "Expected 0 < x < 9, found {}",x);
        assert!( 0 < y && y < 9, "Expected 0 < y < 9, found {}",y);
        Self {x: x, y: y}
    }
    fn get_y_board(&self) -> usize {
        (8 - self.y) as usize
    }
    fn get_x_board(&self) -> usize {
        (self.x - 1) as usize
    }
}

impl Board {
    pub fn new_board() -> Board {
        Board {
            board : [[ Square::Empty ; 8] ; 8],
        }
    }
    pub fn can_move(&self, initial_pos: &Position, final_pos: &Position) -> bool {
        true
    }
    pub fn place_piece(&mut self, piece: Piece, position: &Position){
        let x = position.get_x_board();
        let y = position.get_y_board();
        self.board[y][x] = Square::NonEmpty(piece);
    }
    pub fn place_piece_at(&mut self, piece: Piece, x: u8, y: u8){
        let pos = Position::new_position(x,y);
        self.place_piece(piece, &pos);
    }
    fn remove_piece(&mut self, position: &Position){
        let x = position.get_x_board();
        let y = position.get_y_board();
        self.board[y][x] = Square::Empty;
    }
    pub fn display(&self){
        for row in self.board.iter() {
            for square in row.iter() {
                let char_piece = square.display();
                print!(" | {}",char_piece);
            }
            println!(" |");
        }
    }
    pub fn initial_position(&mut self){
        let mut x:u8 = 1;
        let mut y:u8 = 8;
        let initial_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        for c in initial_fen.chars() {
            match c {
                c if c.is_digit(10) => {
                    let spaces = c.to_digit(10).unwrap() as u8;
                    x += spaces;
                },
                '/' => {
                    y -= 1;
                    x = 1;
                },
                _ => {
                    let piece = Self::piece_from_char(&c);
                    self.place_piece_at(piece,x,y);
                    x += 1;
                },
            };
        };
    }
    fn piece_from_char(char_piece: &char) -> Piece {
        let color =
            if char_piece.is_uppercase() {
                Color::White
            } else {
                Color::Black
        };
        let piece = char_piece.to_lowercase().next().unwrap();
        match piece {
            'k' => Piece::King(color),
            'q' => Piece::Queen(color),
            'b' => Piece::Bishop(color),
            'n' => Piece::Knight(color),
            'r' => Piece::Rook(color),
            'p' => Piece::Pawn(color),
            _ => {
                panic!("{} no se reconoce como pieza!", piece);
            },
        }
    }
}




