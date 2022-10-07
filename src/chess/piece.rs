use std::ops::{Add, Mul, Shr, ShrAssign, BitXor};

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

        println!("{:?} + {:?} = {}", self, rhs, target);

        // if direction becomes negative,
        // it cannot be added because the piece is going out of bounds
        if target < 0 || target >= 64 {
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
            Direction::Up => -8,
            Direction::Down => 8,
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::UpLeft => Direction::Up + Direction::Left,
            Direction::UpRight => Direction::Up + Direction::Right,
            Direction::DownLeft => Direction::Down + Direction::Left,
            Direction::DownRight => Direction::Down + Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
            (Color::Black, P::Preview) => ".",
        }
    }

    pub fn get_paths(&self) -> Vec<Vec<Direction>> {
        match self.class {
            P::Pawn => {
                todo!()
            }
            P::Knight => {
                vec![
                    vec![Direction::Up, Direction::Up, Direction::Left],
                    vec![Direction::Up, Direction::Up, Direction::Right],
                    vec![Direction::Down, Direction::Down, Direction::Left],
                    vec![Direction::Down, Direction::Down, Direction::Right],
                    vec![Direction::Left, Direction::Left, Direction::Up],
                    vec![Direction::Left, Direction::Left, Direction::Down],
                    vec![Direction::Right, Direction::Right, Direction::Up],
                    vec![Direction::Right, Direction::Right, Direction::Down],
                ]
            }
            _ => panic!("invalid piece?"),
        }
    }

}
