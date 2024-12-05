use crate::pieces::{Square, Piece, Color};

pub struct Board {
    board: [[ Square ; 8] ; 8],
    en_pasant: EnPasant,
}
#[derive(Clone, Copy)]
pub struct Position {
    // x, y in [1,8]
    x: u8,
    y: u8,
}
#[derive(Clone, Copy)]
enum EnPasant {
    Enable(Position),
    Disable,
}

impl Position {
    pub fn new_position(x: u8, y: u8) -> Self {
        assert!( 0 < x && x < 9, "Expected 0 < x < 9, found {}",x);
        assert!( 0 < y && y < 9, "Expected 0 < y < 9, found {}",y);
        Self {x: x, y: y}
    }
    pub fn get_x(&self) -> u8 {
        self.x
    }
    pub fn get_y(&self) -> u8 {
        self.y
    }
    fn get_y_board(&self) -> usize {
        (8 - self.y) as usize
    }
    fn get_x_board(&self) -> usize {
        (self.x - 1) as usize
    }
    pub fn is_same_row(&self, position: &Position) -> bool {
        self.y == position.get_y()
    }
    pub fn is_same_column(&self, position: &Position) -> bool {
        self.x == position.get_x()
    }
    pub fn is_same_diagonal(&self, position: &Position) -> bool {
        let distances = self.distances(position);
        distances[0] == distances[1]
    }
    pub fn distances(&self, position: &Position) -> [i32; 2] {
        let distance_x = self.x - position.get_x();
        let distance_x = (distance_x as i32).abs();
        let distance_y = self.y - position.get_y();
        let distance_y = (distance_y as i32).abs();
        [distance_x, distance_y]
    }
}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Board {
    pub fn new_board() -> Board {
        Board {
            board : [[ Square::Empty ; 8] ; 8],
            en_pasant: EnPasant::Disable,
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
                print!(" {}",char_piece);
            }
            println!(" ");
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
                    let piece = 
                        match Self::piece_from_char(c) {
                            Ok(p) => p,
                            Err(error) => panic!("{}", error),
                        };
                    self.place_piece_at(piece,x,y);
                    x += 1;
                },
            };
        };
    }
    fn piece_from_char(char_piece: char) -> Result<Piece, String> {
        Piece::piece_from_char(char_piece)
    }
    fn is_white_in_check(&self) -> bool {
        false
    }
    fn is_black_in_check(&self) -> bool {
        false
    }
    fn find_king(&self, color: Color) -> Position {
        for (row_index, row) in self.board.iter().enumerate() {
            for (col_index, square) in row.iter().enumerate() {
                if let Square::NonEmpty(piece) = square {
                    if Piece::is_king(piece, &color) {
                        return Position::new_position((col_index+1) as u8, (8-row_index) as u8);
                    }
                };
            }
        }
        panic!("No kings on the board!");
    }
    pub fn get_en_pasant(&self) -> Option<Position> {
        match self.en_pasant {
            EnPasant::Enable(position) => Some(position.clone()),
            EnPasant::Disable => None,
        }
    }
}




