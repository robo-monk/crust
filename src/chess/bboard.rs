use super::piece::{Color, Direction, Piece, P};


#[derive(Debug)]
pub struct BBoard {
    white: [u64; 6],
    black: [u64; 6],
}

impl BBoard {
    fn new() -> Self {
        BBoard {
            white: [0; 6],
            black: [0; 6],
        }
    }

    fn place(&mut self, piece: Piece, target: u8) {
        // let mut piece_bit_board = self.get_bboard_of_piece(&piece);
        // piece_bit_board ^= 1 << target;
        // self.mutate_bboard_of_piece(&piece) ^= 1 << target;
        // let i = || {};
        self.mutate_bboard_of_piece(&piece, move |b: u64| {
          b ^1 << target
        });
    }

    // pub fn mutate_bboard_of_piece<F>(&mut self, piece: &Piece, mutation: F) {

    pub fn mutate_bboard_of_piece<F>(&mut self, piece: &Piece, mutation: F) where F: Fn(u64) -> u64 {

        let side_bit_boards = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        let i = match piece.class {
            P::Pawn => 0,
            P::Knight => 1,
            P::Bishop => 2,
            P::Rook => 3,
            P::Queen => 4,
            P::King => 5,
            P::Preview => panic!("preview is not supported"),
        } as usize;

        side_bit_boards[i] = mutation(side_bit_boards[i]);
    }

    pub fn get_bboard_of_piece(&self, piece: &Piece) -> u64 {
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

    fn pprint(&self) {}
}
