use super::piece::{Color, Direction, Piece, P};

#[derive(Debug)]
pub struct BBoard {
    white: [u64; 6],
    black: [u64; 6],
}

impl BBoard {
    pub fn new() -> Self {
        BBoard {
            white: [0; 6],
            black: [0; 6],
        }
    }

    pub fn place(&mut self, piece: Piece, target: u8) {
        // let mut piece_bit_board = self.get_bboard_of_piece(&piece);
        // piece_bit_board ^= 1 << target;
        // self.mutate_bboard_of_piece(&piece) ^= 1 << target;
        // let i = || {};
        self.mutate_bboard_of_piece(&piece, move |b: u64| b | 1 << target);
    }

    // pub fn mutate_bboard_of_piece<F>(&mut self, piece: &Piece, mutation: F) {

    pub fn mutate_bboard_of_piece<F>(&mut self, piece: &Piece, mutation: F)
    where
        F: Fn(u64) -> u64,
    {
        // let side_bit_boards = match piece.color {
        //     Color::White => &mut self.white,
        //     Color::Black => &mut self.black,
        // };

        // let i = match piece.class {
        //     P::Pawn => 0,
        //     P::Knight => 1,
        //     P::Bishop => 2,
        //     P::Rook => 3,
        //     P::Queen => 4,
        //     P::King => 5,
        //     P::Preview => panic!("preview is not supported"),
        // } as usize;
        let side_bit_board = self.get_bboard_of_piece(piece);

        *side_bit_board = mutation(*side_bit_board);
        // side_bit_boards[i] = mutation(side_bit_boards[i]);
    }

    pub fn get_bboard_of_piece(&mut self, piece: &Piece) -> &mut u64 {
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
            P::Preview => panic!("preview is not supported"),
        } as usize;

        &mut side_bit_boards[piece_bit_board_index as usize]
    }

    pub fn pprint(&self) {
        let mut bb = self.white[0].clone();

        for color in [ Color::White, Color::Black ] {
          for class in [P::Pawn, P::Bishop, P::King, P::King, P::Queen] {

          } 
        }

        loop {
          // let index = bb.trailing_zeros();
          // let index = bb.leading_zeros();
          println!("{bb:64b}");
          // let index = bb.trailing_zeros();
          let index = bb.trailing_zeros();
          if index >= 64 { break }
          println!("index > {index}");
          // bb &= 1 << index-1; // switch 1 to zero
          // bb &= 1 << (64 - (index+1));
          bb &= !(1 << index);
          println!("{bb:64b}");
          // let index = bb.trailing_zeros();
          // println!("count of zeros upfron > {index}");
        // self.mutate_bboard_of_piece(&piece, move |b: u64| b | 1 << (64 - (target + 1)));
          // break;
        }
        // bb.reverse_bits();
        // for i in 0..8 {
        //     // change it to get range
        //     let lsb = bb & 1;

        //     if lsb 
        //     bb >>= 1;
        // }
    }
}
