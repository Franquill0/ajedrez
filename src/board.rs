use crate::pieces::{Square, Piece, Color};

pub struct Board {
    board: [[ Square ; 8] ; 8],
    en_pasant: EnPasant,
    white_castle: Castle,
    black_castle: Castle,
    turn: Color,
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

#[derive(Clone, Copy)]
struct Castle {
    long: bool,
    short: bool,
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

impl Castle {
    pub fn is_long_castle_enable(&self) -> bool {
        self.long
    }
    pub fn is_short_castle_enable(&self) -> bool {
        self.short
    }
    pub fn disable_long_castle(&mut self){
        self.long = false;
    }
    pub fn disable_short_castle(&mut self){
        self.short = false;
    }
}

impl Board {
    pub fn new_board() -> Board {
        Board {
            board : [[ Square::Empty ; 8] ; 8],
            en_pasant: EnPasant::Disable,
            white_castle: Castle {long: true, short: true},
            black_castle: Castle {long: true, short: true},
            turn: Color::White,
        }
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
                    if piece.is_king() && piece.get_color() == color {
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
    pub fn can_move(&self, initial_pos: &Position, final_pos: &Position) -> bool {
        if *initial_pos == *final_pos {
            return false;
        }
        let piece = 
            match self.get_piece(initial_pos) {
                Some(p) => p,
                None => return false,
        };
        match piece {
            Piece::Queen(_)  => Self::can_queen_move(initial_pos, final_pos),
            Piece::Rook(_)   => Self::can_rook_move(initial_pos, final_pos),
            Piece::Knight(_) => Self::can_knight_move(initial_pos, final_pos),
            Piece::King(_)   => Self::can_king_move(initial_pos, final_pos),
            Piece::Bishop(_) => Self::can_bishop_move(initial_pos, final_pos),
            Piece::Pawn(_)   => self.can_pawn_move(initial_pos, final_pos),
        }
    }
    fn can_queen_move(initial_pos: &Position, final_pos: &Position) -> bool {
        initial_pos.is_same_row(final_pos) || initial_pos.is_same_column(final_pos) || initial_pos.is_same_diagonal(final_pos)
    }
    fn can_rook_move(initial_pos: &Position, final_pos: &Position) -> bool {
        initial_pos.is_same_row(final_pos) || initial_pos.is_same_column(final_pos)
    }
    fn can_bishop_move(initial_pos: &Position, final_pos: &Position) -> bool {
        initial_pos.is_same_diagonal(final_pos)
    }
    fn can_king_move(initial_pos: &Position, final_pos: &Position) -> bool {
        let distances = initial_pos.distances(final_pos);
        distances[0] + distances[1] <= 2 && distances[0] < 2 && distances[1] < 2
    }
    fn can_knight_move(initial_pos: &Position, final_pos: &Position) -> bool {
        let distances = initial_pos.distances(final_pos);
        distances[0] == 2 && distances[1] == 1 || distances[0] == 1 && distances[1] == 2
    }
    fn can_pawn_move(&self, initial_pos: &Position, final_pos: &Position) -> bool {
        unimplemented!()
    }
    fn get_piece(&self, position: &Position) -> Option<Piece> {
        let x = position.get_x_board();
        let y = position.get_y_board();
        self.board[y][x].get_piece()
    }
}




