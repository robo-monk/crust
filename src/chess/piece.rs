use std::ops::Add;

impl Add for Direction {
  type Output = i64;
  fn add(self, rhs: Self) -> Self::Output {
    self.value() + rhs.value()
  }
}

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
      _ => panic!("invalid direction")
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Up, Down, // and in the end its only round and round
  Left, Right, // and round
  UpLeft, UpRight,
  DownLeft, DownRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
  Black,
  White
}

#[derive(Debug, Clone, Copy)]
pub enum P {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,

  Preview
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
  pub class: P,
  pub color: Color
}

impl Piece {
  pub fn new(class: P, color: Color) -> Self {
    Piece {
      class, color
    }
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

}
