use std::thread::{self, JoinHandle};

// use super::board::{Board}
use super::board::{Board, Move, self};
use super::piece::{Direction, Color};

trait Piece {
    fn new(color: Color) -> Self;
    fn symbol(&self) -> &str;
    fn get_paths(&self, index: usize, board: &Board) -> Vec<Vec<Direction>>;
    fn get_moves(&self, index: usize, board: &Board) -> Vec<Move>;

    fn get_rank(board_index: usize) -> usize {
        board_index / 8
    }

    fn get_file(board_index: usize) -> usize {
        board_index % 8
    }

    fn get_diagonal(board_index: usize) -> (usize, usize) {
      (Self::get_file(board_index), Self::get_rank(board_index))
    }
}

pub struct Queen {
  color: Color
}

impl Piece for Queen {
    fn symbol(&self) -> &str {
      match self.color {
        Color::White => "♕",
        Color::Black => "♛"
      }
    }

    fn get_paths(&self, index: usize, board: &Board) -> Vec<Vec<Direction>> {
        todo!()
    }

    fn get_moves(&self, index: usize, board: &Board) -> Vec<Move> {
      if board.turn != self.color { return vec![] }
      let mut target_squares: Vec<JoinHandle<Vec<usize>>> = vec![];

      for direction in [Direction::Down, Direction::Left] {
        // let b = board.clone();
        // target_squares.push(thread::spawn(move || {
          let mut _target_squares: Vec<usize> = vec![];

          // go to that direction until you hit a wall / piece
          loop {
            let target_index = index + direction;
            if target_index.is_none() { break }

            // let target = board;
            // let target = b.get_index(target_index.unwrap());
            let target = board.get_index(target_index.unwrap());

            // board

            // direction
            // break;
          }
          // todo!()
        // }));
      }

      todo!()
    }

    fn new(color: Color) -> Self {
      Queen { color }
    }
}

// fn test() {
//   let q: Queen = Piece::new(Color::Black);
//   let pieces: [dyn Piece; 1] = [q];
//   let a = q.symbol();
// }
