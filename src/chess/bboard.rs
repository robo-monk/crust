use super::board::Board;
use super::piece::{Color, Direction, Piece, P};

const H_FILE: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
const A_FILE: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;

const RANK_1: u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
const RANK_2: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;

const RANK_7: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
const RANK_8: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
// const H_FILE: u64 = 0b0000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

fn rank_mask(sq: &u64) -> u64 {
    0xff << (sq & 56)
}

fn file_mask(sq: &u64) -> u64 {
    0x0101010101010101 << (sq & 7)
}

fn rotate(i: u64, v: i32) -> u64 {
    if v.is_negative() {
        i.rotate_right(v.abs() as u32)
    } else {
        i.rotate_left(v as u32)
    }
}

fn occluded_fill(mut gen: u64, mut pro: u64, direction: Direction) -> u64 {
    let r: i32 = direction.value() as i32; // {+-1,7,8,9}
    pro &= direction.avoid_wrap();

    gen |= pro & rotate(gen, r);
    pro &= rotate(pro, r);
    gen |= pro & rotate(gen, 2 * r);
    pro &= rotate(pro, (2 * r));
    gen | pro & rotate(gen, (4 * r))
    // gen
}

// U64 shiftOne (U64 b, int dir8)
fn shift_one(b: u64, direction: Direction) -> u64 {
    let r = direction.value() as i32;
    rotate(b, r) & direction.avoid_wrap()
}

fn sliding_attacks(slider: u64, empty: u64, direction: Direction) -> u64 {
    let fill = occluded_fill(slider, empty, direction);
    shift_one(fill, direction)
}

fn bishop_attacks(_bb: u64, empty: u64) -> u64 {
    sliding_attacks(_bb, empty, Direction::UpLeft)
        | sliding_attacks(_bb, empty, Direction::UpRight)
        | sliding_attacks(_bb, empty, Direction::DownRight)
        | sliding_attacks(_bb, empty, Direction::DownLeft)
}

fn rook_attacks(_bb: u64, empty: u64) -> u64 {
    sliding_attacks(_bb, empty, Direction::Up)
        | sliding_attacks(_bb, empty, Direction::Right)
        | sliding_attacks(_bb, empty, Direction::Down)
        | sliding_attacks(_bb, empty, Direction::Left)
}

fn queen_attacks(_bb: u64, empty: u64) -> u64 {
    bishop_attacks(_bb, empty) | rook_attacks(_bb, empty)
}

// U64 rankMask(int sq) {return  C64(0xff) << (sq & 56);}

// U64 fileMask(int sq) {return C64(0x0101010101010101) << (sq & 7);}

// U64 diagonalMask(int sq) {
//    const U64 maindia = C64(0x8040201008040201);
//    int diag =8*(sq & 7) - (sq & 56);
//    int nort = -diag & ( diag >> 31);
//    int sout =  diag & (-diag >> 31);
//    return (maindia >> sout) << nort;
// }

// U64 antiDiagMask(int sq) {
//    const U64 maindia = C64(0x0102040810204080);
//    int diag =56- 8*(sq&7) - (sq&56);
//    int nort = -diag & ( diag >> 31);
//    int sout =  diag & (-diag >> 31);
//    return (maindia >> sout) << nort;
// }

#[derive(Debug, Clone)]
pub struct BBoard {
    white: [u64; 7],
    black: [u64; 7],
    turn: Color,
}

impl BBoard {
    pub fn new() -> Self {
        BBoard {
            turn: Color::White,
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
        // print!("unplace> ");
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
        // println!("before bit mutation {side_bit_board:64b}");
        *side_bit_board = mutation(*side_bit_board);
        // println!("after bit mutation {side_bit_board:64b}");
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

    pub fn get_piece_indeces(&self, piece: &Piece) -> Vec<u64> {
        let mut bb = self.get_bboard_of_piece(&piece).clone();
        let mut indeces = Vec::new();

        loop {
            let index = bb.trailing_zeros();

            if index >= 64 {
                break;
            }

            // switch to zero
            bb &= !(1 << index);

            indeces.push(index as u64);
        }

        indeces
    }
    pub fn pprint(&mut self) {
        let mut board: Board = Board::new();

        for color in [Color::Black, Color::White] {
            for class in [
                P::Pawn,
                P::Bishop,
                P::King,
                P::Queen,
                P::Rook,
                P::Knight,
                P::Preview,
            ] {
                let piece = Piece { color, class };
                let mut bb = self.get_bboard_of_piece(&piece).clone();
                loop {
                    let index = bb.trailing_zeros();

                    if index >= 64 {
                        break;
                    }
                    bb &= !(1 << index);
                    board._set_square(index as usize, Some(piece));
                }
            }
        }

        board.print();
        let available_moves = self.count_available_moves();
        println!("> {:?} to play", self.turn);
        println!("> {available_moves} available moves...");

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
        // self.mutate_bboard_of_piece(&Piece::new(P::Preview, piece.color), |bb: u64| {
        self.mutate_bboard_of_piece(&Piece::new(P::Preview, Color::White), |bb: u64| {
            bb | moves
            // A_FILE
            // RANK_1
        });
        self.pprint();
    }

    pub fn get_available_targets(&self, color: Color) -> [u64; 6] {
        let mut targets: [u64; 6] = [0; 6];

        for (i, class) in [P::Pawn, P::Bishop, P::King, P::Queen, P::Rook, P::Knight]
            .iter()
            .enumerate()
        {
            let piece = Piece {
                color,
                class: *class,
            };
            let available_moves = self.get_available_captures(&piece);
            targets[i] = available_moves;
            // let mut bb = self.get_bboard_of_piece(&piece).clone();
            // loop {
            //     let index = bb.trailing_zeros();

            //     if index >= 64 {
            //         break;
            //     }
            //     bb &= !(1 << index);
            //     // board._set_square(index as usize, Some(piece));
            // }
        }
        targets
    }

    pub fn count_available_moves(&self) -> u32 {
      let targets = self.get_available_targets(self.turn);
      targets.iter().fold(0, |count, t| count + t.count_ones())
    }

    pub fn get_available_captures(&self, piece: &Piece) -> u64 {
        let bb = self.get_bboard_of_piece(&piece);

        let us_bitmap = (match piece.color {
            Color::White => self.white,
            Color::Black => self.black,
        })
        .iter()
        .fold(0, |bb, pice_bb| bb | pice_bb);
        let them_bitmap = (match piece.color {
            Color::White => self.black,
            Color::Black => self.white,
        })
        .iter()
        .fold(0, |bb, pice_bb| bb | pice_bb);

        let empty = !(us_bitmap | them_bitmap);

        match piece.class {
            P::Pawn => {
                let c = if piece.color == Color::Black { -1 } else { 1 };
                let attacks = ((bb >> 9 * c & !A_FILE) | (bb >> 7 * c & !H_FILE)) & them_bitmap; // moves forward

                let first_move_rank = if piece.color == Color::Black {
                    RANK_7
                } else {
                    RANK_2
                };

                let moves = ((bb >> 8 * c) | (bb & first_move_rank) >> 16) & !them_bitmap; // moves forward

                println!("not moved pawns {:64b}", (bb & RANK_1));
                println!("attacks {attacks:64b}");
                println!("enemy bitmap {them_bitmap:64b}");

                let fill = occluded_fill(bb, empty, Direction::Down);
                (fill & moves) | attacks
                // fill
                // (A_FILE & !self.white[0]) |
                // (RANK_3 & !self.white[0]) |
                // (RANK_2 & !self.white[0]) |
                // (RANK_1 & !self.white[0])
                //   & !(us_bitmap | them_bitmap)

                // rank_mask(2)
                // bb & RANK_2
                // enemy_bitmap
            }

            P::Queen => queen_attacks(bb, empty) & !us_bitmap,
            P::King => 0,
            P::Rook => rook_attacks(bb, empty) & !us_bitmap,
            P::Bishop => bishop_attacks(bb, empty) & !us_bitmap,
            P::Knight => 0,
            _ => todo!(),
        }
    }
}
