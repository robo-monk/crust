use super::piece::{Color, Direction, Piece, P};
use super::board::{Board};

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

    pub fn register_move(&mut self, from: &str, to: &str, piece: Piece) {
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

    pub fn get_bboard_of_piece(&mut self, piece: &Piece) -> u64 {
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
            P::Preview => panic!("preview is not supported"),
        } as usize;

        &mut side_bit_boards[piece_bit_board_index as usize]
    }

    pub fn pprint(&mut self) {
      let mut board: Board = Board::new();

        for color in [Color::White, Color::Black] {
            for class in [P::Pawn, P::Bishop, P::King, P::Queen, P::Rook, P::Knight] {
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
}
