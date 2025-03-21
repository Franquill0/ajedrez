use crate::pieces::{Square, Piece, Color};

pub struct Board {
    board: [[ Square ; 8] ; 8],
    en_pasant: EnPasant,
    white_castle: Castle,
    black_castle: Castle,
    turn: Color,
}
struct BoardIter {
    x: i8,
    y: i8,
    dir: Direction,
}
#[derive(Clone,Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}
#[derive(Clone, Copy)]
pub struct Position {
    // x, y in [1,8]
    x: i8,
    y: i8,
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

impl Direction {
    fn step_x(&self) -> i8 {
        match self {
            Self::Right     |
            Self::UpRight   |
            Self::DownRight  => 1,
            Self::Left      |
            Self::UpLeft    |
            Self::DownLeft   => -1,
            _                => 0,
        }
    }
    fn step_y(&self) -> i8 {
        match self {
            Self::Up        |
            Self::UpRight   |
            Self::UpLeft     => 1,
            Self::Down      |
            Self::DownRight |
            Self::DownLeft   => -1,
            _                => 0,
        }
    }
    fn get_direction_from_to(initial_position: Position, final_position: Position) -> Option<Self> {
        if initial_position == final_position {
            return None;
        }
        let is_right_to = initial_position.is_right_to(final_position);
        let is_above = initial_position.is_above(final_position);
        if initial_position.is_same_row(final_position) {
            if is_right_to {
                Some(Self::Left)
            } else {
                Some(Self::Right)
            }
        } else if initial_position.is_same_column(final_position) {
            if is_above {
                Some(Self::Down)
            } else {
                Some(Self::Up)
            }
        } else if initial_position.is_same_diagonal(final_position) {
            if is_above {
                if is_right_to {
                    Some(Self::DownLeft)
                } else {
                    Some(Self::DownRight)
                }
            } else {
                if is_right_to {
                    Some(Self::UpLeft)
                } else {
                    Some(Self::UpRight)
                }
            }
        } else {
            None
        }
    }
}

impl BoardIter {
    fn new(initial_position: Position, direction: Direction) -> Self {
        let initial_x = initial_position.get_x();
        let initial_y = initial_position.get_y();
        Self {x: initial_x, y: initial_y, dir: direction}
    }
    fn is_in_range(x: i8, y: i8) -> bool {
        0 < x && x < 9 && 0 < y && y < 9
    }
    pub fn get_x(&self) -> i8 {
        self.x
    }
    pub fn get_y(&self) -> i8 {
        self.y
    }

}

impl Iterator for BoardIter {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        let next_x: i8 = self.x as i8 + self.dir.step_x();
        let next_y: i8 = self.y as i8 + self.dir.step_y();
        if Self::is_in_range(next_x, next_y){
            self.x = next_x;
            self.y = next_y;
            Some((self.x, self.y))
        } else {
            None
        }
    }
}

impl Position {
    fn new_position(x: i8, y: i8) -> Self {
        assert!( 0 < x && x < 9, "Expected 0 < x < 9, found {}",x);
        assert!( 0 < y && y < 9, "Expected 0 < y < 9, found {}",y);
        Self {x: x, y: y}
    }
    fn get_x(&self) -> i8 {
        self.x
    }
    fn get_y(&self) -> i8 {
        self.y
    }
    fn get_y_board(&self) -> usize {
        (8 - self.y) as usize
    }
    fn get_x_board(&self) -> usize {
        (self.x - 1) as usize
    }
    fn is_same_row(&self, position: Position) -> bool {
        self.y == position.get_y()
    }
    fn is_same_column(&self, position: Position) -> bool {
        self.x == position.get_x()
    }
    fn is_same_diagonal(&self, position: Position) -> bool {
        let distances = self.distances(position);
        distances[0] == distances[1]
    }
    fn distances(&self, position: Position) -> [i32; 2] {
        let distance_x = self.x - position.get_x();
        let distance_x = (distance_x as i32).abs();
        let distance_y = self.y - position.get_y();
        let distance_y = (distance_y as i32).abs();
        [distance_x, distance_y]
    }
    fn is_valid_position(x: i8, y:i8) -> bool {
        0 < x && x < 9 && 0 < y && y < 9
    }
    fn is_right_to(&self, final_position: Position) -> bool {
        self.get_x() > final_position.get_x()
    }
    fn is_left_to(&self, final_position: Position) -> bool {
        self.get_x() < final_position.get_x()
    }
    fn is_above(&self, final_position: Position) -> bool {
        self.get_y() > final_position.get_y()
    }
    fn is_below(&self, final_position: Position) -> bool {
        self.get_y() < final_position.get_y()
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
    pub fn place_piece(&mut self, piece: Piece, position: Position){
        let x = position.get_x_board();
        let y = position.get_y_board();
        self.board[y][x] = Square::NonEmpty(piece);
    }
    pub fn place_piece_at(&mut self, piece: Piece, x: i8, y: i8){
        let pos = Position::new_position(x,y);
        self.place_piece(piece, pos);
    }
    fn remove_piece(&mut self, position: Position){
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
        let mut x:i8 = 1;
        let mut y:i8 = 8;
        let initial_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        for c in initial_fen.chars() {
            match c {
                c if c.is_digit(10) => {
                    let spaces = c.to_digit(10).unwrap() as i8;
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
    pub fn is_white_in_check(&self) -> bool {
        self.is_in_check(Color::White)
    }
    pub fn is_black_in_check(&self) -> bool {
        self.is_in_check(Color::Black)
    }
    fn is_in_check(&self, player: Color) -> bool {
        let king = self.find_king(player);
        return self.is_in_check_horizontaly_or_verticaly(king) ||
        self.is_in_check_by_pawn(king)   ||
        self.is_in_check_diagonaly(king) ||
        self.is_in_check_by_knight(king);
    }
    fn is_in_check_by_knight(&self, king_position: Position) -> bool {
        let king_opposite_color = self.get_piece(king_position).unwrap().get_color().opposite();
        for position in self.get_possible_moves_of_knight_from(king_position).iter() {
            match self.get_piece(*position) {
                Some(piece) => if piece.is_knight_of_color(king_opposite_color) {return true;},
                None => {},
            };
        }
        false
    }
    fn get_possible_moves_of_knight_from(&self, position: Position) -> Vec<Position> {
        let pos_x = position.get_x();
        let pos_y = position.get_y();
        let moves = vec![
            (pos_x+1, pos_y+2),
            (pos_x+2, pos_y+1),
            (pos_x-1, pos_y-2),
            (pos_x-2, pos_y-1),
            (pos_x+1, pos_y-2),
            (pos_x-1, pos_y+2),
            (pos_x+2, pos_y-1),
            (pos_x-2, pos_y+1),
        ];
        let mut possible_moves:Vec<Position> = Vec::new();
        for (x, y) in moves.iter() {
            if Position::is_valid_position(*x,*y) {
                possible_moves.push(Position::new_position(*x,*y));
            }
        }
        return possible_moves;
    }
    fn is_in_check_by_pawn(&self, king_position: Position) -> bool {
        match self.get_piece(king_position) {
            Some(piece) => 
                if !piece.is_king() {
                    panic!("Expected King, found {}", piece.display_full_name());
                },
            None => panic!("Expected King, found no piece!"),
        }
        let king_color = self.get_piece(king_position).unwrap().get_color();
        match king_color {
            Color::White => self.is_white_king_in_check_by_pawn(king_position),
            Color::Black => self.is_black_king_in_check_by_pawn(king_position),
        }
    }
    fn is_white_king_in_check_by_pawn(&self, king_position:Position) -> bool {
        self.is_king_in_check_by_pawn_in_direction(king_position, Direction::UpRight) || 
        self.is_king_in_check_by_pawn_in_direction(king_position, Direction::UpLeft)
    }
    fn is_black_king_in_check_by_pawn(&self, king_position:Position) -> bool {
        self.is_king_in_check_by_pawn_in_direction(king_position, Direction::DownRight) || 
        self.is_king_in_check_by_pawn_in_direction(king_position, Direction::DownLeft)
    }
    fn is_king_in_check_by_pawn_in_direction(&self, king_position: Position, direction: Direction) -> bool {
        let x_pawn_position = king_position.get_x() + direction.step_x();
        let y_pawn_position = king_position.get_y() + direction.step_y();
        if Position::is_valid_position(x_pawn_position, y_pawn_position) {
            let pawn_position = Position::new_position(x_pawn_position, y_pawn_position);
            if let Some(piece) = self.get_piece(pawn_position) {
                let opposite_king_color = self.get_piece(king_position).unwrap().get_color().opposite();
                return piece.is_pawn_of_color(opposite_king_color);
            }
        }
        false
    }
    fn is_in_check_diagonaly(&self, king_position: Position) -> bool {
        let king_color = self.get_piece(king_position).unwrap().get_color();
        let directions = [Direction::UpRight, Direction::UpLeft, Direction::DownRight, Direction::DownLeft];
        for direction in directions.iter() {
            let iterator = BoardIter::new(king_position, *direction);
            for (col, row) in iterator {
                let position = Position::new_position(col, row);
                if let Some(piece) = self.get_piece(position) {
                    if piece.is_queen_or_bishop_of_color(king_color.opposite()){
                        return true;
                    } else {
                        break;
                    }
                }
            }
        }
        false
    }
    fn is_in_check_horizontaly_or_verticaly(&self, king_position: Position) -> bool {
        let king_color = self.get_piece(king_position).unwrap().get_color();
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        for direction in directions.iter() {
            let iterator = BoardIter::new(king_position, *direction);
            for (col, row) in iterator {
                let position = Position::new_position(col, row);
                if let Some(piece) = self.get_piece(position) {
                    if piece.is_queen_or_rook_of_color(king_color.opposite()){
                        return true;
                    } else {
                        break;
                    }
                }
            }
        }
        false
    }
    fn find_king(&self, color: Color) -> Position {
        for (row_index, row) in self.board.iter().enumerate() {
            for (col_index, square) in row.iter().enumerate() {
                if let Square::NonEmpty(piece) = square {
                    if piece.is_king() && piece.get_color() == color {
                        return Position::new_position((col_index+1) as i8, (8-row_index) as i8);
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
    pub fn can_move(&self, initial_pos: Position, final_pos: Position) -> bool {
        if initial_pos == final_pos {
            return false;
        }
        let piece = 
            match self.get_piece(initial_pos) {
                Some(p) => p,
                None => return false,
        };
        match piece {
            Piece::Queen(_)  => self.can_queen_move(initial_pos, final_pos),
            Piece::Rook(_)   => self.can_rook_move(initial_pos, final_pos),
            Piece::Knight(_) => self.can_knight_move(initial_pos, final_pos),
            Piece::King(_)   => self.can_king_move(initial_pos, final_pos),
            Piece::Bishop(_) => self.can_bishop_move(initial_pos, final_pos),
            Piece::Pawn(_)   => self.can_pawn_move(initial_pos, final_pos),
        }
    }
    fn can_queen_move(&self, initial_pos: Position, final_pos: Position) -> bool {
        initial_pos.is_same_row(final_pos) || initial_pos.is_same_column(final_pos) || initial_pos.is_same_diagonal(final_pos)
    }
    fn can_rook_move(&self, initial_pos: Position, final_pos: Position) -> bool {
        initial_pos.is_same_row(final_pos) || initial_pos.is_same_column(final_pos)
    }
    fn can_bishop_move(&self, initial_pos: Position, final_pos: Position) -> bool {
        initial_pos.is_same_diagonal(final_pos)
    }
    fn can_king_move(&self, initial_pos: Position, final_pos: Position) -> bool {
        let distances = initial_pos.distances(final_pos);
        distances[0] + distances[1] <= 2 && distances[0] < 2 && distances[1] < 2
    }
    fn can_knight_move(&self, initial_pos: Position, final_pos: Position) -> bool {
        let distances = initial_pos.distances(final_pos);
        distances[0] == 2 && distances[1] == 1 || distances[0] == 1 && distances[1] == 2
    }
    fn can_pawn_move(&self,  initial_pos: Position, final_pos: Position) -> bool {
        unimplemented!()
    }
    fn get_piece(&self, position: Position) -> Option<Piece> {
        let x = position.get_x_board();
        let y = position.get_y_board();
        self.board[y][x].get_piece()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_position_not_in_check(){
        let mut my_board = Board::new_board();
        my_board.initial_position();
        assert!(!my_board.is_white_in_check());
        assert!(!my_board.is_black_in_check());
    }
}


