use super::board::{Board, CastlingRights};
use super::piece::{Color, Direction, Piece, P};
use rand::Rng;
use std::fmt::Debug; // 0.8.5
                     // use serde::{Serialize, Deserialize};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// use serde::*;
// use serde_derive::{Serialize, Deserialize};
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

macro_rules! console_log {
    ($($t:tt)*) => (println!("{}", format_args!($($t)*).to_string()))
    // ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    pub from: u32,
    pub target: u32,
    pub piece: Piece,
    pub captures: Option<Piece>,
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("from", &BBoard::i_to_sq(self.from))
            .field("target", &BBoard::i_to_sq(self.target))
            .field("piece", &self.piece)
            .field("captures", &self.captures)
            .finish()
    }
}

pub const H_FILE: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
pub const A_FILE: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;

pub const RANK_1: u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
pub const RANK_2: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;

pub const RANK_7: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
pub const RANK_8: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

pub const PIECES: [P; 6] = [P::Pawn, P::Knight, P::Bishop, P::Rook, P::Queen, P::King];
pub const PIECES_PERV: [P; 7] = [
    P::Pawn,
    P::Knight,
    P::Bishop,
    P::Rook,
    P::Queen,
    P::King,
    P::Preview,
];
// const H_FILE: u64 = 0b0000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
pub fn loop_through_indeces<F>(mut bb: u64, mut reducer: F)
where
    // F: Fn(u32) -> (),
    F: FnMut(u32) -> (),
{
    loop {
        let index = bb.trailing_zeros();

        if index >= 64 {
            break;
        }

        reducer(index);
        bb &= !(1 << index);
    }
}

fn get_rank(board_index: usize) -> usize {
    8 - board_index / 8
}

fn get_file(board_index: usize) -> usize {
    board_index % 8
}

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
pub fn index_mask(i: u32) -> u64 {
    (1 as u64) << i
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
    sliding_attacks(_bb, empty, Direction::DownLeft)
        | sliding_attacks(_bb, empty, Direction::DownRight)
        | sliding_attacks(_bb, empty, Direction::UpRight)
        | sliding_attacks(_bb, empty, Direction::UpLeft)
}

fn rook_attacks(_bb: u64, empty: u64) -> u64 {
    sliding_attacks(_bb, empty, Direction::Down)
        | sliding_attacks(_bb, empty, Direction::Right)
        | sliding_attacks(_bb, empty, Direction::Up)
        | sliding_attacks(_bb, empty, Direction::Left)
}

fn queen_attacks(_bb: u64, empty: u64) -> u64 {
    bishop_attacks(_bb, empty) | rook_attacks(_bb, empty)
}
fn knight_attacks(knights: u64) -> u64 {
    let l1 = (knights >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2 = (knights >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1 = (knights << 1) & 0xfefefefefefefefe;
    let r2 = (knights << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}

fn king_attacks(bb: u64) -> u64 {
    let attacks: u64 = Direction::Right.shift_once(bb) | Direction::Left.shift_once(bb);
    let _bb = bb | attacks;
    attacks | Direction::Down.shift_once(_bb) | Direction::Up.shift_once(_bb)
}

fn king_queen_castle(bb: u64, empty_and_not_under_attack: u64, cr: &CastlingRights) -> u64 {
    if cr.queen {
        let first_step = Direction::Left.shift_once(bb) & empty_and_not_under_attack;
        Direction::Left.shift_once(first_step) & empty_and_not_under_attack
    } else {
        0
    }
}

fn king_king_castle(bb: u64, empty_and_not_under_attack: u64, cr: &CastlingRights) -> u64 {
    if cr.king {
        (Direction::Right
            .shift_once((Direction::Right.shift_once(bb)) & empty_and_not_under_attack))
            & (Direction::Right.shift_twice(bb) & empty_and_not_under_attack)
    } else {
        0
    }
}

fn pawn_attacks(bb: u64, color: Color) -> u64 {
    match color {
        Color::White => (Direction::UpRight.shift_once(bb) | Direction::UpLeft.shift_once(bb)),
        Color::Black => (Direction::DownRight.shift_once(bb) | Direction::DownLeft.shift_once(bb)),
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BBoard {
    white: [u64; 7],
    black: [u64; 7],
    pub turn: Color,
    pub en_passant_target: Option<usize>,
    pub white_cr: CastlingRights,
    pub black_cr: CastlingRights,
}

impl BBoard {
    pub fn new() -> Self {
        BBoard {
            turn: Color::White,
            white: [0; 7],
            black: [0; 7],
            // todo
            /*
            if 563: & (them_bitamp | en_passant_mask)
            411: if m.target = enpassant square && m.pice is Pawn
              capture above / below pawn depending on color

            castling rights

            king capture -> if king is captured the game is over
            */
            en_passant_target: None,

            white_cr: CastlingRights {
                queen: true,
                king: true,
            },
            black_cr: CastlingRights {
                queen: true,
                king: true,
            },
        }
    }

    pub fn from_fen(fen: &String) -> Self {
        let board = Board::from_fen(fen);
        let mut bboard = BBoard::new();

        bboard.white_cr = board.white_cr;
        bboard.black_cr = board.black_cr;
        bboard.en_passant_target = board.en_passant;
        bboard.turn = board.turn;

        for (i, sq) in board.squares.iter().enumerate() {
            if sq.is_none() {
                continue;
            }
            let piece = sq.unwrap();
            dbg!(i, piece);

            bboard.place(piece, i as u8);
        }

        bboard
    }
    pub fn shift_turn(&mut self) -> () {
        self.turn = self.not_turn();
    }

    pub fn i_to_sq(i: u32) -> String {
        let file = get_file(i as usize);
        let rank = get_rank(i as usize);
        let file_letter = char::from_u32((file + 97 as usize).try_into().unwrap()).unwrap();
        format!("{file_letter}{rank}")
    }

    pub fn get_board(&self) -> Board {
        let mut board = Board::new();

        board.white_cr = self.white_cr.clone();
        board.black_cr = self.black_cr.clone();
        board.en_passant = self.en_passant_target;
        board.turn = self.turn;

        for color in [Color::White, Color::Black] {
            for (_, class) in PIECES.iter().enumerate() {
                let piece = Piece {
                    color,
                    class: *class,
                };

                let bb = self.get_bboard_of_piece(&piece);

                loop_through_indeces(bb, |i| board.squares[i as usize] = Some(piece))
            }
        }

        board
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

        self.shift_turn();
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

        let piece_bit_board_index = BBoard::get_piece_bb_index(piece.class);
        side_bit_boards[piece_bit_board_index as usize]
    }

    pub fn get_piece_bb_index(class: P) -> usize {
        match class {
            P::Pawn => 0,
            P::Knight => 1,
            P::Bishop => 2,
            P::Rook => 3,
            P::Queen => 4,
            P::King => 5,
            P::Preview => 6,
        }
    }

    pub fn get_mut_bboard_of_piece(&mut self, piece: &Piece) -> &mut u64 {
        let side_bit_boards = match piece.color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        };

        let piece_bit_board_index = BBoard::get_piece_bb_index(piece.class);
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
            for class in PIECES_PERV {
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
        println!("\n-------------------");
        println!("  {:?} to play", self.turn);
        println!("  eval(4): {:?}", self.evaluate(self.turn));
        println!("  {available_moves} available moves...");
    }

    pub fn preview(&mut self, moves: u64) {
        self.mutate_bboard_of_piece(&Piece::new(P::Preview, Color::White), |bb: u64| bb | moves);
    }

    pub fn preview_moves_of(&mut self, sq: &str, class: P, color: Color) {
        let i = BBoard::parse_sq(sq);
        let moves = self.get_available_moves_at_index(i as u32, &Piece { class, color });
        self.preview(moves);
        self.pprint();
    }

    pub fn preview_attackers(&mut self) {
        let attackers = self.attack_map_of(self.not_turn());
        self.preview(attackers);
        self.pprint();
    }

    pub fn get_sqs_attackers(&self, sqs: u64, them_color: Color) -> u64 {
        let us_bitmap = self.get_side_bitmap(them_color.not());
        // let them_bitmap = self.get_side_bb();
        // let them_bitmap = us;
        // let them_bitmap = self.them_bitmap();
        // let empty = !(us_bitmap | them_bitmap);

        // bishop_attacks(sqs, )
        todo!()
    }

    pub fn get_attackers_of_bb(bb: u64, us: u64, them: u64) -> u64 {
        let empty = !(us | them);

        let possible_attacks = queen_attacks(
            bb, empty
        ) | knight_attacks(bb);

        possible_attacks & them
    }

    pub fn attack_map_of(&self, color: Color) -> u64 {
        let us_bitmap = self.get_side_bitmap(color);
        let them_bitmap = self.get_side_bitmap(color.not());

        self._attack_map_of(color, us_bitmap, them_bitmap)
    }

    pub fn _attack_map_of(&self, color: Color, us_bitmap: u64, them_bitmap: u64) -> u64 {
        // let us_bitmap = 0;
        // let them_bitmap = them_bitmap;

        // let them_bitmap = self.them_bitmap();
        // let empty = !(us_bitmap | them_bitmap);
        // let empty = !(us_bitmap | them_bitmap);
        let empty = !(us_bitmap | them_bitmap);
        // let full_empty = 0xffffffffffffffff;

        let op_attacks = pawn_attacks(
            self.get_bboard_of_piece(&Piece {
                color,
                class: P::Pawn,
            }),
            color,
        ) | bishop_attacks(
            self.get_bboard_of_piece(&Piece {
                color,
                class: P::Bishop,
            }),
            empty,
        ) | rook_attacks(
            self.get_bboard_of_piece(&Piece {
                color,
                class: P::Rook,
            }),
            empty,
        ) | queen_attacks(
            self.get_bboard_of_piece(&Piece {
                color,
                class: P::Queen,
            }),
            empty,
        ) | king_attacks(
            self.get_bboard_of_piece(&Piece {
                color,
                class: P::King,
            }),
            // us_bitmap,
        ) | knight_attacks(self.get_bboard_of_piece(&Piece {
            color,
            class: P::Knight,
        }));

        // op_attacks & !them_bitmap
        op_attacks
    }

    pub fn preview_moves(&mut self, piece: &Piece) {
        let moves = self.get_available_moves(piece);
        self.preview(moves);
        self.pprint();
    }

    pub fn get_available_targets(&self, color: Color) -> [u64; 6] {
        let mut targets: [u64; 6] = [0; 6];

        for (i, class) in PIECES.iter().enumerate() {
            let piece = Piece {
                color,
                class: *class,
            };
            let available_moves = self.get_available_moves(&piece);
            targets[i] = available_moves;
        }

        targets
    }

    pub fn make_good_move(&mut self, search_depth: u32) -> () {
        let m = self.search_good_move(search_depth);

        println!(
            "AGENT > search in depth {search_depth} => chose good move > {:?}",
            &m
        );

        self.push_unchecked_move(&m);
    }

    pub fn make_random_move(&mut self) -> () {
        let mut _moves: Vec<Move> = vec![];

        self.loop_through_moves_and_captures(self.turn, |m| {
            _moves.push(m);
        });

        let move_i = rand::thread_rng().gen_range(0.._moves.len());
        let rmove = _moves.get(move_i).unwrap();

        // if rmove.piece.class == P::King && rmove.from.abs_diff(rmove.target) > 8 {
        //     println!("CASTLE")
        // }

        println!(
            "AGENT > from {} possible actions => chose #{move_i} > {:?}",
            _moves.len(),
            &rmove
        );

        self.push_unchecked_move(rmove);
        // self.make_unchecked_move(target as u8, from as u8, piece);
        // println!("{}", num);
        // println!("can take {} actions", possible_actions.len());
    }

    fn alpha_beta_max(&self, mut alpha: i32, beta: i32, depth: u32) -> i32 {
        if depth == 0 {
            return self.evaluate(self.turn);
        }

        let moves = self.get_moves_and_captures(self.turn);
        for m in moves.iter() {
            let mut c = self.clone();
            let result = c.push_unchecked_move(&m);

            if result.is_err() {
                continue;
            }

            let score = c.alpha_beta_min(alpha, beta, depth - 1);
            if score >= beta {
                return beta;
                // break;
                // return -1
            };

            if score > alpha {
                alpha = score
            }
        }

        return alpha;
    }

    fn alpha_beta_min(&self, alpha: i32, mut beta: i32, depth: u32) -> i32 {
        if depth == 0 {
            return -self.evaluate(self.turn);
        }

        let moves = self.get_moves_and_captures(self.turn);
        for m in moves.iter() {
            let mut c = self.clone();
            let result = c.push_unchecked_move(m);
            if result.is_err() {
                continue;
            }

            let score = c.alpha_beta_max(alpha, beta, depth - 1);

            if score <= alpha {
                return alpha;
            }

            if score < beta {
                beta = score
            }
        }

        return beta;
    }

    pub fn eval_side_score(&self, side: Color) -> i32 {
        let mut score = 0;
        let bba = self.get_side_bba(side);

        for piece_i in 0..6 {
            let piece = Piece::new(PIECES[piece_i], side);
            let piece_bb = bba[piece_i];

            let mult = match piece.class {
                P::Pawn => 1,
                P::Knight | P::Bishop => 3,
                P::Rook => 5,
                P::Queen => 9,
                P::King => 10,
                P::Preview => 0,
            };

            score += piece_bb.count_ones() * mult;
        }
        score as i32
    }

    pub fn search_good_move(&self, depth: u32) -> Move {
        // let check = self.is_in_check(self.turn);
        let check = BBoard::is_in_check(self.get_bboard_of_piece(&Piece {
            class: P::King,
            color: self.turn
        }), self.get_side_bitmap(self.turn), self.get_side_bitmap(self.turn.not()));

        let turn = self.turn;
        console_log!("> Is [{turn:?}] in check? {:?}", check);

        // let mut best_score: i32 = -999999999;
        let mut best_score = -f32::INFINITY as i32;
        let mut best_score = f32::INFINITY as i32;

        let mut best_move = Move {
            from: 0,
            target: 0,
            captures: None,
            piece: Piece {
                class: P::Pawn,
                color: Color::Black,
            },
        };

        let score = self.alpha_beta_max(-f32::INFINITY as i32, f32::INFINITY as i32, depth);
        println!(">> score of alpha beta max is {score}",);

        console_log!(">> socre of alpha beta max is {score}");

        self.loop_through_moves_and_captures(self.turn, |m| {
            let mut c = self.clone();
            let res = c.push_unchecked_move(&m);
            if res.is_ok() {
                let score = c.alpha_beta_max(-f32::INFINITY as i32, f32::INFINITY as i32, depth);
                if score <= best_score {
                    best_score = score;
                    best_move = m;
                }
            }
        });

        best_move
    }

    pub fn evaluate(&self, side: Color) -> i32 {
        let perspective = match side {
            Color::Black => -1,
            Color::White => 1,
        };

        let white_score = self.eval_side_score(Color::White) as i32;
        let black_score = self.eval_side_score(Color::Black) as i32;
        (white_score - black_score) * perspective
    }

    pub fn get_moves_and_captures(&self, side: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        self.loop_through_moves_and_captures(side, |m| moves.push(m));
        moves
    }

    pub fn loop_through_moves_and_captures<F>(&self, side: Color, mut reducer: F)
    where
        F: FnMut(Move) -> (),
    {
        let us_bba = self.get_side_bba(side);
        let them_bba = self.get_side_bba(side.not());
        let them = self.get_side_bitmap(side.not());

        for piece_i in 0..6 {
            let piece = Piece::new(PIECES[piece_i], side);
            let piece_bb = us_bba[piece_i];

            loop_through_indeces(piece_bb, |from| {
                let moves = self.get_available_moves_at_index(from, &piece); // turn agnostic
                let captures = moves & them;

                loop_through_indeces(moves & !captures, |target| {
                    let m = Move {
                        from,
                        target,
                        piece,
                        captures: None,
                    };

                    reducer(m);
                });

                loop_through_indeces(captures, |target| {
                    for i in 0..6 {
                        let captured_piece = Piece::new(PIECES[i], side.not());
                        let piece_bb = self.get_bboard_of_piece(&captured_piece);
                        if (piece_bb & index_mask(target)) > 0 {
                            let m = Move {
                                from,
                                target,
                                piece,
                                captures: Some(captured_piece),
                            };

                            reducer(m);
                        }
                    }
                });
            });
        }
    }

    pub fn count_ply_moves(&mut self, depth: u32) -> u32 {
        if depth <= 0 {
            return 1;
        }

        let mut move_count = 0;

        self.loop_through_moves_and_captures(self.turn, |m| {
            let mut c = self.clone();
            if c.push_unchecked_move(&m).is_ok() {
                move_count += c.count_ply_moves(depth - 1);
            };
        });

        move_count
    }

    pub fn not_turn(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn get_side_bitmap(&self, side: Color) -> u64 {
        self.get_side_bba(side)
            .iter()
            .fold(0, |bb, piece_bb| bb | piece_bb)
    }

    pub fn get_side_bba(&self, side: Color) -> [u64; 7] {
        match side {
            Color::Black => self.black,
            Color::White => self.white,
        }
    }

    pub fn get_them_bb_array(&self) -> [u64; 7] {
        self.get_side_bba(self.turn.not())
    }

    pub fn get_turns_bb_array(&self) -> [u64; 7] {
        self.get_side_bba(self.turn)
    }

    pub fn count_available_moves(&self) -> u32 {
        let targets = self.get_available_targets(self.turn);
        targets.iter().fold(0, |count, t| count + t.count_ones())
    }

    pub fn count_available_moves_at_index(&self, i: u32, piece: &Piece) -> u32 {
        self.get_available_moves_at_index(i, piece).count_ones()
    }

    pub fn get_available_moves_at_index(&self, i: u32, piece: &Piece) -> u64 {
        let bb = 1 << i;

        if bb & self.get_bboard_of_piece(piece) == 0 {
            panic!("no piece found at {i}");
        }

        self.get_available_moves_of_piece_type(bb, piece)
    }

    pub fn get_available_moves(&self, piece: &Piece) -> u64 {
        let bb = self.get_bboard_of_piece(&piece);
        self.get_available_moves_of_piece_type(bb, piece)
    }

    fn update_board_state_from_move(&mut self, m: &Move) -> Result<(), ()> {
        // HANDLE A CAPTURE
        if m.captures.is_some() {
            let mut capture_index = m.target as u8;

            // HANDLE EN PASSANT CAPTURE
            if m.piece.is_pawn()
                && self.en_passant_target.is_some()
                && m.captures.unwrap().is_pawn()
                && m.target == self.en_passant_target.unwrap() as u32
            {
                capture_index = match m.piece.color {
                    Color::White => Direction::Down.value() + m.target as i64,
                    Color::Black => Direction::Up.value() + m.target as i64, // Color::White => Direction::Up(self.white[0])
                } as u8;
            };

            // println!("------------------------------------------------CAPTURE");
            // self.pprint();
            self.unplace(m.captures.unwrap(), capture_index);
            // self.pprint();
        }

        if m.piece.is_pawn() && m.from.abs_diff(m.target) == 16 {
            // SET EN PASSANT SQ
            let en_passant_index = match m.piece.color {
                Color::White => Direction::Down.value() + m.target as i64,
                Color::Black => Direction::Up.value() + m.target as i64,
            };

            self.en_passant_target = Some(en_passant_index as usize);
        } else if self.en_passant_target.is_some() {
            self.en_passant_target = None;
        }

        let _cr = self.get_castling_rights(m.piece.color);
        if m.piece.class == P::King && (_cr.king || _cr.queen) {
            // if self_clone.is_none() {
            //     self_clone = Some(self.clone())
            // }

            let (cr_from, cr_to) = match (m.piece.color, m.target) {
                // white king side castling
                (Color::White, 62) => {
                    // self.white_cr.king = false;
                    (63, 63 + Direction::Left.value() + Direction::Left.value())
                }
                // white queen side castling
                (Color::White, 58) => {
                    // self.white_cr.queen = false;
                    (
                        56,
                        56 + Direction::Right.value()
                            + Direction::Right.value()
                            + Direction::Right.value(),
                    )
                }
                // black king side castling
                (Color::Black, 6) => {
                    // self.black_cr.king = false;
                    (7, 7 + Direction::Left.value() + Direction::Left.value())
                }
                // black queen side castling
                (Color::Black, 2) => {
                    // self.black_cr.queen = false;
                    (
                        0,
                        0 + Direction::Right.value()
                            + Direction::Right.value()
                            + Direction::Right.value(),
                    )
                }
                _ => (-1, -1),
            };

            if cr_from > -1 && cr_to > -1 {
                let rook = Piece {
                    color: m.piece.color,
                    class: P::Rook,
                };
                self.make_unchecked_move(cr_from as u8, cr_to as u8, rook);
                self.shift_turn();
            }

            const DISABLE_CASTLING_RIGHTS: CastlingRights = CastlingRights {
                king: false,
                queen: false,
            };

            match m.piece.color {
                Color::White => self.white_cr = DISABLE_CASTLING_RIGHTS,
                Color::Black => self.black_cr = DISABLE_CASTLING_RIGHTS,
            };
        }

        // if rook gets moves make sure cr is disabled
        if m.piece.class == P::Rook {
            match (m.piece.color, m.target) {
                // white king side castling
                (Color::White, 63) => self.white_cr.king = false,
                (Color::White, 56) => self.white_cr.queen = false,
                (Color::Black, 7) => self.black_cr.king = false,
                (Color::Black, 0) => self.black_cr.queen = false,
                _ => (),
            };
        }

        // if rook gets captured make sure cr is disabled
        if m.captures.is_some() && m.captures.unwrap().is_rook() {
            match (m.captures.unwrap().color, m.target) {
                // white king side castling
                (Color::White, 63) => self.white_cr.king = false,
                (Color::White, 56) => self.white_cr.queen = false,
                (Color::Black, 7) => self.black_cr.king = false,
                (Color::Black, 0) => self.black_cr.queen = false,
                _ => (),
            };
        }

        let check = BBoard::is_in_check(self.get_bboard_of_piece(&Piece {
            class: P::King,
            color: m.piece.color 
        }), self.get_side_bitmap(m.piece.color), self.get_side_bitmap(m.piece.color.not()));


        // if self.is_in_check(m.piece.color) {
        if check {
            return Err(());
        } else {
            return Ok(());
        }
    }

    fn get_castling_rights(&self, color: Color) -> &CastlingRights {
        match color {
            Color::White => &self.white_cr,
            Color::Black => &self.black_cr,
        }
    }

    pub fn push_unchecked_move(&mut self, m: &Move) -> Result<(), ()> {
        self.make_unchecked_move(m.from as u8, m.target as u8, m.piece);
        self.update_board_state_from_move(m)
    }

    pub fn count_attackers_of_square(&self, bb: u64, color: Color) -> u32 {
        todo!()
    }

    pub fn is_in_check(king: u64, us: u64, them: u64) -> bool {
        let attackers = BBoard::get_attackers_of_bb(king, us, them);
        attackers != 0
    }

    // pub fn is_in_check(&self, color: Color) -> bool {
    //     (self.attack_map_of(color.not())
    //         & self.get_bboard_of_piece(&Piece {
    //             color,
    //             class: P::King,
    //         }))
    //         != 0
    // }

    pub fn get_available_moves_of_piece_type(&self, bb: u64, piece: &Piece) -> u64 {
        let us_bitmap = self.get_side_bitmap(piece.color);
        let them_bitmap = self.get_side_bitmap(piece.color.not());

        let empty = !(us_bitmap | them_bitmap);

        match piece.class {
            P::Pawn => {
                let mut en_passant_bitmap = 0;
                if self.turn == piece.color {
                    en_passant_bitmap = &(1 as u64) << self.en_passant_target.unwrap_or(0);
                    // en_passant_bitmap = 0;
                }

                let attacks = pawn_attacks(bb, piece.color) & (them_bitmap | en_passant_bitmap); // moves forward
                                                                                                 // return them_bitmap & attacks;
                                                                                                 // let attacks = pawn_attacks(bb, piece.color) ; // moves forward

                if piece.color == Color::Black {
                    let first_move_rank = RANK_7;

                    // let moves = Direction::Down.shift_once(bb)
                    //     | (bb & first_move_rank) << 16 & !them_bitmap; // moves forward

                    let moves = (Direction::Down.shift_once(bb) | (bb & first_move_rank) << 16)
                        & !them_bitmap;

                    let fill = occluded_fill(bb, empty, Direction::Down);

                    ((fill & moves) | attacks) & !us_bitmap
                } else {
                    let first_move_rank = RANK_2;

                    let moves = (Direction::Up.shift_once(bb) | (bb & first_move_rank) >> 16)
                        & !them_bitmap; // moves forward

                    let fill = occluded_fill(bb, empty, Direction::Up);
                    ((fill & moves) | attacks) & !us_bitmap
                }
            }
            P::Queen => queen_attacks(bb, empty) & !us_bitmap,
            P::King => {
                let cr = match piece.color {
                    Color::White => &self.white_cr,
                    Color::Black => &self.black_cr,
                };

                let enemy_attack_map = self.attack_map_of(piece.color.not());
                let emtpy_and_not_under_attack = empty & !enemy_attack_map;
                // let emtpy_and_not_under_attack = !self.attack_map_of(piece.color.not());
                // let emtpy_and_not_under_attack = self.attack_map_of(piece.color.not());

                // return emtpy_and_not_under_attack;
                // return (king_attacks(bb, empty));
                // return enemy_attack_map;
                // return king_attacks(bb) & !us_bitmap;
                ((king_attacks(bb) & !enemy_attack_map)
                    | king_king_castle(bb, emtpy_and_not_under_attack, cr)
                    | king_queen_castle(bb, emtpy_and_not_under_attack, cr))
                    & !us_bitmap
            }
            P::Rook => rook_attacks(bb, empty) & !us_bitmap,
            P::Bishop => bishop_attacks(bb, empty) & !us_bitmap,
            P::Knight => knight_attacks(bb) & !us_bitmap,
            _ => todo!(),
        }

        // let enemy_attack_map = self.attack_map_of(piece.color.not());
        // let enemy_attack_map = self._attack_map_of(
        //     piece.color.not(),
        //     us_bitmap | (us_bitmap ^ moves),
        //     them_bitmap,
        // );
        // let king_bb = self.get_bboard_of_piece(&Piece {
        //     class: P::King,
        //     color: piece.color,
        // });

        // return !(enemy_attack_map & king_bb);
        // return moves & !(enemy_attack_map & king_bb);

        // moves
    }

    pub fn find_piece_at_index(&self, target: u32, side: Color) -> Piece {
        let bba = self.get_side_bba(side);
        // let target = BBoard::parse_sq(sq);
        for i in 0..6 {
            let piece = Piece::new(PIECES[i], side);

            // if captured_piece.color.eq(&piece.color) {

            let piece_bb = bba[i];

            // if (piece_bb & index_mask(target)) != 0 {
            if (piece_bb & index_mask(target)) != 0 {
                return piece;
                // break;
            }
        }

        return panic!("no piece there");
    }

    pub fn serialize(&self) -> String {
        // self
        let serialized = serde_json::to_string(self).unwrap();

        // Prints serialized = {"x":1,"y":2}
        println!("serialized = {}", serialized);
        serialized
    }

    pub fn from_serialization(val: &str) -> BBoard {
        let b: BBoard = serde_json::from_str(val).unwrap();
        b
        // serde_json::Deserializer
    }
}
