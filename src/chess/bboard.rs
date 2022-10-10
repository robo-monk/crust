use super::piece::{Color, Direction, Piece, P};
use super::board::{Board};

const H_FILE: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
const A_FILE: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
// const H_FILE: u64 = 0b0000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

#[derive(Debug)]
pub struct BBoard {
    white: [u64; 7],
    black: [u64; 7],
}

impl BBoard {
    pub fn new() -> Self {
        BBoard {
            white: [0; 7],
            black: [0; 7],
        }
    }

    pub fn parse_sq(n: &str) -> u8 {
      Board::parse_notation(&n.to_string()).unwrap() as u8 
    }
    pub fn place(&mut self, piece: Piece, target: u8) {
        self.mutate_bboard_of_piece(&piece, |b: u64| b | 1 << target);
    }

    pub fn unplace(&mut self, piece: Piece, index: u8) {
      print!("unplace> ");
        self.mutate_bboard_of_piece(&piece, |b: u64| b & !(1 << index));
    }

    pub fn make_unchecked_move(&mut self, from: u8, to: u8, piece: Piece) {
      self.unplace(piece, from);
      self.place(piece, to);
    }

    pub fn register_unchecked_move(&mut self, from: &str, to: &str, piece: Piece) {
      self.unplace(piece, BBoard::parse_sq(from));
      self.place(piece, BBoard::parse_sq(to));
      // self.place(piece, to);
    }

    // pub fn mutate_bboard_of_piece<F>(&mut self, piece: &Piece, mutation: F) {

    pub fn mutate_bboard_of_piece<F>(&mut self, piece: &Piece, mutation: F)
    where
        F: Fn(u64) -> u64,
    {
        let side_bit_board = self.get_mut_bboard_of_piece(piece);
        println!("before bit mutation {side_bit_board:64b}");
        *side_bit_board = mutation(*side_bit_board);
        println!("after bit mutation {side_bit_board:64b}");
    }

    pub fn get_bboard_of_piece(&self, piece: &Piece) -> u64 {
        let side_bit_boards = match piece.color {
            Color::White => &self.white,
            Color::Black => &self.black,
        };

        let piece_bit_board_index = match piece.class {
            P::Pawn => 0,
            P::Knight => 1,
            P::Bishop => 2,
            P::Rook => 3,
            P::Queen => 4,
            P::King => 5,
            P::Preview => 6,
        } as usize;

        side_bit_boards[piece_bit_board_index as usize]
    }

    pub fn get_mut_bboard_of_piece(&mut self, piece: &Piece) -> &mut u64 {
        let side_bit_boards = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        let piece_bit_board_index = match piece.class {
            P::Pawn => 0,
            P::Knight => 1,
            P::Bishop => 2,
            P::Rook => 3,
            P::Queen => 4,
            P::King => 5,
            P::Preview => 6,
        } as usize;

        &mut side_bit_boards[piece_bit_board_index as usize]
    }

    pub fn pprint(&mut self) {
      let mut board: Board = Board::new();

        for color in [Color::Black, Color::White] {
            for class in [P::Pawn, P::Bishop, P::King, P::Queen, P::Rook, P::Knight, P::Preview] {
                let piece = Piece { color, class };
                let mut bb = self.get_bboard_of_piece(&piece).clone();
                loop {
                    // let index = bb.trailing_zeros();
                    // let index = bb.leading_zeros();
                    // println!("{bb:64b}");
                    // let index = bb.trailing_zeros();
                    let index = bb.trailing_zeros();

                    if index >= 64 {
                        break;
                    }
                    // println!("index > {index}");
                    // bb &= 1 << index-1; // switch 1 to zero
                    // bb &= 1 << (64 - (index+1));
                    // switch to zero
                    bb &= !(1 << index);
                    // dbg!(index, piece);

                    board._set_square(index as usize, Some(piece));
                    // println!("{bb:64b}");
                    // let index = bb.trailing_zeros();
                    // println!("count of zeros upfron > {index}");
                    // self.mutate_bboard_of_piece(&piece, move |b: u64| b | 1 << (64 - (target + 1)));
                    // break;
                }
            }
        }

        board.print();

        // bb.reverse_bits();
        // for i in 0..8 {
        //     // change it to get range
        //     let lsb = bb & 1;

        //     if lsb
        //     bb >>= 1;
        // }
    }
    
    pub fn preview_moves(&mut self, piece: &Piece) {
      let moves = self.get_available_captures(piece);
      // let bb =  self.get_bboard_of_piece(piece); 
      // let preview = self.get_bboard_of_piece(&Piece::new(P::Preview, piece.color));
      self.mutate_bboard_of_piece(&Piece::new(P::Preview, piece.color), |bb: u64| {
        bb | moves
        // A_FILE
      });
      self.pprint();
    }

    pub fn get_available_captures(&self, piece: &Piece) -> u64 {
      let bb = self.get_bboard_of_piece(&piece); 

      let enemy_bitmap = self.black.iter().fold(0, |bb, pice_bb| bb | pice_bb);
      //  iter.reduce(|accum, item| {
        // if accum >= item { accum } else { item }
    // })
      match piece.class {
        P::Pawn => {
          // let attacks = (bb << 9 & !A_FILE) | (bb << 7 & !A_FILE);
          // let attacks = (bb << 9 & !A_FILE) | (bb << 7 & !A_FILE);
          // let attacks = (bb >> 8 & !A_FILE); // moves forward
          let attacks = 
            ((bb >> 9 & !A_FILE) | (bb >> 7 & !A_FILE)) & enemy_bitmap;// moves forward
          println!("attacks {attacks:64b}");
          println!("enemy bitmap {enemy_bitmap:64b}");
          attacks
          // enemy_bitmap
        }
        _ => todo!()
      }
    }



}
