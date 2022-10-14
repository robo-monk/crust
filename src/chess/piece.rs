use super::board::Board;
use super::bboard::*;
use std::ops::{Add, BitXor, Mul, Shr, ShrAssign};

// A >> B (can A pass through B?)
impl Shr for Piece {
    type Output = bool;
    fn shr(self, rhs: Self) -> Self::Output {
        if self.is_knight() {
            true
        } else {
            false
        }
        //  todo!()
    }
}

impl BitXor for Piece {
    type Output = bool;
    fn bitxor(self, rhs: Self) -> Self::Output {
        // println!("XXXXXXXXXXXXXXXXXXXXXOR {:?} {:?}", self, rhs);
        self.color != rhs.color
    }
}

impl Add for Direction {
    type Output = i64;
    fn add(self, rhs: Self) -> Self::Output {
        self.value() + rhs.value()
    }
}

impl Add<Direction> for usize {
    type Output = Option<usize>;

    // add a direction
    fn add(self, rhs: Direction) -> Self::Output {
        let target = self as isize + rhs.value() as isize;

        let self_rank = Piece::get_rank(self);
        let target_rank = Piece::get_rank(target.abs() as usize);
        let self_diag = Piece::get_diagonal(self);
        let target_diag = Piece::get_diagonal(target.abs() as usize);
        // println!(
        //     "{:?} + {:?} = {} [selfrank: {}, trank: {}, selfdiago: {:?}, tardiago: {:?}]",
        //     self,
        //     rhs,
        //     target,
        //     Piece::get_rank(self),
        //     Piece::get_rank(target.abs() as usize),
        //     Piece::get_diagonal(self),
        //     Piece::get_diagonal(target.abs() as usize) // self / 8,
        //                                                // target / 8
        // );

        // if direction becomes negative,
        // it cannot be added because the piece is going out of bounds
        if target < 0
            || target >= 64
            || (rhs == Direction::Left || rhs == Direction::Right) && self_rank != target_rank
            || (rhs == Direction::DownLeft
                || rhs == Direction::DownRight
                || rhs == Direction::UpLeft
                || rhs == Direction::UpRight)
                && (self_diag.0.abs_diff(target_diag.0) != 1
                    || self_diag.1.abs_diff(target_diag.1) != 1)
        {
            None
        } else {
            Some(target.abs() as usize)
        }
    }
}

// impl Mul<Direction> for i32 {
//   type Output = i64;

//   fn mul(self, mut rhs: Direction) -> Self::Output {
//     let mut out: i64 = 0;

//     for _ in 0..self {
//       out =
//     }
//     out

//   }
// }

impl Direction {
    pub fn value(&self) -> i64 {
        match self {
            Direction::DownRight => 9,
            Direction::Right => 1,
            Direction::UpRight => -7,
            Direction::Up => -8,
            Direction::UpLeft => -9,
            Direction::Left => -1,
            Direction::DownLeft => 7,
            Direction::Down => 8,

            // Direction::Up => 8,
            // Direction::Left => -1,
            // Direction::Right => 1,
            // Direction::DownLeft => Direction::Down + Direction::Left, // 7
            // Direction::UpLeft => Direction::Up + Direction::Left, // -9
            // Direction::UpRight => Direction::Up + Direction::Right, // -7
            _ => panic!("invalid direction"),
        }
    }

    pub fn shift_twice(&self, b: u64) -> u64 {
        self.shift_once(self.shift_once(b))
    }
    pub fn shift_once(&self, b: u64) -> u64 {
        match self {
            Direction::DownRight => (b << 9) & !A_FILE,
            Direction::Right => (b << 1) & !A_FILE,
            Direction::UpRight => (b >> 7) & !A_FILE,
            Direction::Up => b >> 8,
            Direction::UpLeft =>  (b >> 9) & !H_FILE,
            Direction::Left => (b << 1) & !H_FILE,
            Direction::DownLeft => (b << 7) & !H_FILE,
            Direction::Down => b << 8,
            _ => panic!("invalid direction"),
        }
    }

    pub fn avoid_wrap(&self) -> u64 {
        match self {
            Direction::DownRight => 0xfefefefefefefe00,
            Direction::Right => 0xfefefefefefefefe,
            Direction::UpRight => 0x00fefefefefefefe,
            Direction::Up => 0x00ffffffffffffff,
            Direction::UpLeft => 0x007f7f7f7f7f7f7f,
            Direction::Left => 0x7f7f7f7f7f7f7f7f,
            Direction::DownLeft => 0x7f7f7f7f7f7f7f00,
            Direction::Down => 0xffffffffffffff00,

        }
    }

    pub fn range_comb(dir: Vec<Direction>, i: usize) -> Vec<Vec<Direction>> {
        let combination: &mut Vec<Direction> = &mut Vec::new();
        let mut combinations: Vec<Vec<Direction>> = Vec::new();

        for _ in 1..i {
            combination.extend(&dir);
            combinations.push(combination.clone());
        }
        combinations
        // todo!()
        // (1..i).into_iter().map(|i| vec![dir; i]).collect()
    }
    pub fn range(dir: Direction, i: usize) -> Vec<Vec<Direction>> {
        (1..i).into_iter().map(|i| vec![dir; i]).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down, // and in the end its only round and round
    Left,
    Right, // and round
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn not(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum P {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,

    Preview,
}


#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub class: P,
    pub color: Color,
}

impl Piece {
    pub fn new(class: P, color: Color) -> Self {
        Piece { class, color }
    }

    pub fn get_rank(board_index: usize) -> usize {
        board_index / 8
    }

    pub fn get_file(board_index: usize) -> usize {
        board_index % 8
    }

    pub fn get_diagonal(board_index: usize) -> (usize, usize) {
        (Piece::get_file(board_index), Piece::get_rank(board_index))
    }

    pub fn is(&self, class: P) -> bool {
        self.class == class
    }

    pub fn is_queen(&self) -> bool {
        self.is(P::Queen)
    }
    pub fn is_bishop(&self) -> bool {
        self.is(P::Bishop)
    }
    pub fn is_rook(&self) -> bool {
        self.is(P::Rook)
    }
    pub fn is_king(&self) -> bool {
        self.is(P::King)
    }
    pub fn is_pawn(&self) -> bool {
        self.is(P::Pawn)
    }
    pub fn is_knight(&self) -> bool {
        self.is(P::Knight)
    }

    pub fn is_color(&self, color: Color) -> bool {
        self.color == color
    }

    pub fn is_sliding(&self) -> bool {
        self.is_bishop() || self.is_rook() || self.is_queen()
    }

    pub fn can_capture(&self, other: &Piece) -> bool {
        self.color != other.color && !other.is_king()
    }

    pub fn symbol(&self) -> &str {
        match (self.color, self.class) {
            (Color::White, P::Pawn) => "♙",
            (Color::White, P::Knight) => "♘",
            (Color::White, P::Bishop) => "♗",
            (Color::White, P::Rook) => "♖",
            (Color::White, P::Queen) => "♕",
            (Color::White, P::King) => "♔",
            (Color::White, P::Preview) => "x",

            (Color::Black, P::Pawn) => "♟",
            (Color::Black, P::Knight) => "♞",
            (Color::Black, P::Bishop) => "♝",
            (Color::Black, P::Rook) => "♜",
            (Color::Black, P::Queen) => "♛",
            (Color::Black, P::King) => "♚",
            (Color::Black, P::Preview) => "x",
        }
    }

    pub fn get_paths(&self, index: usize, board: &Board) -> Vec<Vec<Direction>> {
        match self.class {
            P::Pawn => {
                let piece = board.get_index(index).unwrap();
                let rank = Piece::get_rank(index);
                let first_move: bool = piece.is_color(Color::White) && rank == 6
                    || piece.is_color(Color::Black) && rank == 1;

                let dir = match piece.color {
                    Color::Black => Direction::Up,
                    Color::White => Direction::Down,
                };

                if first_move {
                    vec![vec![dir, dir], vec![dir]]
                } else {
                    vec![vec![dir]]
                }
            }
            P::King => vec![
                vec![Direction::Down],
                vec![Direction::Up],
                vec![Direction::Left],
                vec![Direction::Right],
            ],
            P::Rook => vec![
                vec![Direction::Down],
                vec![Direction::Up],
                vec![Direction::Left],
                vec![Direction::Right],
            ],
            P::Bishop => vec![
                vec![Direction::DownRight],
                vec![Direction::DownLeft],
                vec![Direction::UpRight],
                vec![Direction::UpLeft],
            ],
            P::Queen => vec![
                vec![Direction::Up],
                vec![Direction::Down],
                vec![Direction::Left],
                vec![Direction::Right],
                vec![Direction::DownRight],
                vec![Direction::DownLeft],
                vec![Direction::UpRight],
                vec![Direction::UpLeft],
            ],
            P::Knight => {
                vec![
                    vec![Direction::Down, Direction::Down, Direction::Left],
                    vec![Direction::Down, Direction::Down, Direction::Right],
                    vec![Direction::Up, Direction::Up, Direction::Left],
                    vec![Direction::Up, Direction::Up, Direction::Right],
                    vec![Direction::Left, Direction::Left, Direction::Down],
                    vec![Direction::Left, Direction::Left, Direction::Up],
                    vec![Direction::Right, Direction::Right, Direction::Down],
                    vec![Direction::Right, Direction::Right, Direction::Up],
                ]
            }
            _ => panic!("invalid piece?"),
        }
    }
}
