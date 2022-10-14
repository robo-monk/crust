use super::board::{Board, CastlingRights};
use super::piece::{Color, Direction, Piece, P};
use rand::Rng; // 0.8.5

#[derive(Clone, Copy, Debug)]
struct Move {
    from: u32,
    target: u32,
    piece: Piece,
    captures: Option<Piece>,
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

fn king_attacks(bb: u64, empty: u64) -> u64 {
    let attacks: u64 = Direction::Right.shift_once(bb) | Direction::Left.shift_once(bb);
    let _bb = bb | attacks;
    attacks | Direction::Down.shift_once(_bb) | Direction::Up.shift_once(_bb)
}

fn king_queen_castle(bb: u64, empty_and_not_under_attack: u64, cr: &CastlingRights) -> u64 {
    if cr.queen {
        (Direction::Left.shift_once((Direction::Left.shift_once(bb)) & empty_and_not_under_attack))
            & (Direction::Left.shift_twice(bb) & empty_and_not_under_attack)
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
        Color::White => (Direction::DownRight.shift_once(bb) | Direction::DownLeft.shift_once(bb)),
        Color::Black => (Direction::DownRight.shift_once(bb) | Direction::DownLeft.shift_once(bb)),
    }
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
    pub fn shift_turn(&mut self) -> () {
        self.turn = self.not_turn();
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

    pub fn _capture(&mut self, from: &str, to: &str, class: P, capture: P) {
        self.push_unchecked_move(Move {
            from: BBoard::parse_sq(from) as u32,
            target: BBoard::parse_sq(to) as u32,
            piece: Piece {
                class,
                color: self.turn,
            },
            captures: Some(Piece {
                class: capture,
                color: self.not_turn(),
            }),
        });
        // self.unplace(piece, BBoard::parse_sq(from));
        // self.place(piece, BBoard::parse_sq(to));
        // self.place(piece, to);
    }
    pub fn _move(&mut self, from: &str, to: &str, class: P) {
        self.push_unchecked_move(Move {
            from: BBoard::parse_sq(from) as u32,
            target: BBoard::parse_sq(to) as u32,
            piece: Piece {
                class,
                color: self.turn,
            },
            captures: None,
        });
        // self.unplace(piece, BBoard::parse_sq(from));
        // self.place(piece, BBoard::parse_sq(to));
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

        if self.en_passant_target.is_some() {
            // board._set_square(
            //     self.en_passant_target.unwrap(),
            //     Some(Piece {
            //         class: P::Preview,
            //         color: Color::White,
            //     }),
            // );
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

    pub fn preview(&mut self, moves: u64) {
        self.mutate_bboard_of_piece(&Piece::new(P::Preview, Color::White), |bb: u64| bb | moves);
    }

    pub fn preview_moves_of(&mut self, sq: &str, class: P) {
        let i = BBoard::parse_sq(sq);
        let moves = self.get_available_moves_at_index(
            i as u32,
            &Piece {
                class,
                color: self.turn,
            },
        );
        self.preview(moves);
        self.pprint();
    }

    pub fn preview_attackers(&mut self) {
        let attackers = self.attack_map_of(self.not_turn());
        self.preview(attackers);
        self.pprint();
    }

    pub fn get_sqs_attackers(&self, sqs: u64, them_color: Color) -> u64 {
        let us_bitmap = self.get_side_bb(them_color.not());
        // let them_bitmap = self.get_side_bb();
        // let them_bitmap = us;
        // let them_bitmap = self.them_bitmap();
        // let empty = !(us_bitmap | them_bitmap);

        // bishop_attacks(sqs, )
        todo!()
    }

    pub fn attack_map_of(&self, color: Color) -> u64 {
        let us_bitmap = self.get_side_bb(color.not());
        let them_bitmap = self.get_side_bb(color);
        // let us_bitmap = 0;
        // let them_bitmap = them_bitmap;

        // let them_bitmap = self.them_bitmap();
        let empty = !(us_bitmap | them_bitmap);

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
            empty,
        ) | knight_attacks(self.get_bboard_of_piece(&Piece {
            color,
            class: P::Knight,
        }));

        op_attacks & !them_bitmap
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

    pub fn make_random_move(&mut self) -> () {
        let targets = self.get_available_targets(self.turn);

        let bb = self.get_turns_bb_array();
        let them = self.them_bitmap();
        // maybe this is slow, convert to for in
        // get_available_moves_at_index
        // for (i, class) in PIECES.iter().enumerate() {

        let mut move_count = 0;
        let mut _moves: Vec<Move> = vec![];
        let mut _captures: Vec<Move> = vec![];

        for piece_i in 0..6 {
            let piece = Piece::new(PIECES[piece_i], self.turn);
            let piece_bb = bb[piece_i];
            // println!("| START [{:?}] > {i}", piece);
            // println!("| {:64b}", piece_bb);
            //  self.get_available_moves_at_index()
            // let i_mask = 1 << i;
            // fn loop_through_indeces(bb: u64, reducer: f)
            loop_through_indeces(piece_bb, |from| {
                // println!("|> - {:?} > {i}", piece);
                let moves = self.get_available_moves_at_index(from, &piece);
                let captures = moves & them;

                loop_through_indeces(moves & !captures, |target| {
                    _moves.push(Move {
                        from,
                        target,
                        piece,
                        captures: None,
                    });
                });

                loop_through_indeces(captures, |target| {
                    for (i, piece_bb) in self.get_them_bb_array().iter().enumerate() {
                        if i == 6 {
                            break;
                        }; //gross, skip preview looping

                        if (piece_bb & (1 << target)) > 0 {
                            // println!("captures a {:?}", PIECES[i as usize]);
                            let captured_piece_type = PIECES[i as usize];

                            let captures = Piece {
                                class: captured_piece_type,
                                color: self.not_turn(),
                            };

                            _captures.push(Move {
                                from,
                                target,
                                piece,
                                captures: Some(captures),
                            });
                            break;
                        }
                    }
                });
            });
        }

        let possible_actions = vec![_moves, _captures].concat();
        // let possible_actions = vec![_captures].concat();

        // / Generate random number in the range [0, 99]
        let action_i = rand::thread_rng().gen_range(0..possible_actions.len());
        let rmove = possible_actions.get(action_i).unwrap();
        if rmove.piece.class == P::King && rmove.from.abs_diff(rmove.target) > 8 {
            println!("CASTLE")
        }
        println!(
            "AGENT > from {} possible actions => chose #{action_i} > {:?}",
            possible_actions.len(),
            &rmove
        );
        self.push_unchecked_move(*rmove);
        // self.make_unchecked_move(target as u8, from as u8, piece);
        // println!("{}", num);
        // println!("can take {} actions", possible_actions.len());
    }

    // pub push_unch

    // self.make_unchecked_move(target as u8, from as u8, piece);
    pub fn count_ply_moves(&mut self, depth: u32) -> u32 {
        if depth <= 0 {
            return 1;
        }

        let targets = self.get_available_targets(self.turn);

        let bb = self.get_turns_bb_array();
        let them = self.them_bitmap();
        // maybe this is slow, convert to for in
        // get_available_moves_at_index
        // for (i, class) in PIECES.iter().enumerate() {

        let mut move_count = 0;
        for piece_i in 0..6 {
            let piece = Piece::new(PIECES[piece_i], self.turn);
            let piece_bb = bb[piece_i];
            // println!("| START [{:?}] > {i}", piece);
            // println!("| {:64b}", piece_bb);
            //  self.get_available_moves_at_index()
            // let i_mask = 1 << i;
            // fn loop_through_indeces(bb: u64, reducer: f)
            loop_through_indeces(piece_bb, |from| {
                // println!("|> - {:?} > {i}", piece);
                let moves = self.get_available_moves_at_index(from, &piece);
                // move_count += moves.count_ones();

                let captures = moves & them;
                loop_through_indeces(moves & !captures, |target| {
                    // println!("---> - from: ({i}) > to ({ii})");
                    // self.make_unchecked_move()
                    // let mut subboard = self;
                    self.make_unchecked_move(from as u8, target as u8, piece);
                    move_count += self.count_ply_moves(depth - 1);
                    self.make_unchecked_move(target as u8, from as u8, piece);
                    // self.make_unchecked_move(i as u8, ii as u8, piece);
                    // self.clone().mutate_bboard_of_piece()
                });

                loop_through_indeces(captures, |target| {
                    // println!("---> - from: ({i}) > to ({ii})");
                    // self.make_unchecked_move()
                    // let mut subboard = self;
                    for (i, piece_bb) in self.get_them_bb_array().iter().enumerate() {
                        if (piece_bb & (1 << target)) > 0 {
                            // println!("captures a {:?}", PIECES[i as usize]);
                            let captured_piece_type = PIECES[i as usize];
                            let captured_piece = Piece {
                                class: captured_piece_type,
                                color: self.not_turn(),
                            };
                            // bb[i]

                            self.unplace(captured_piece, target as u8);
                            self.make_unchecked_move(from as u8, target as u8, piece);
                            move_count += self.count_ply_moves(depth - 1);
                            self.make_unchecked_move(target as u8, from as u8, piece);
                            self.place(captured_piece, target as u8);
                            break;
                        }
                    }
                    // println!("capture");
                    // self.make_unchecked_move(i as u8, ii as u8, piece);
                    // self.clone().mutate_bboard_of_piece()
                });
            });

            // println!("| END {:?} > {i} \n", piece);
        }

        move_count
        // let available_moves = self.count_available_moves();
        // 0
    }

    pub fn not_turn(&self) -> Color {
        match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn get_side_bb(&self, side: Color) -> u64 {
        (match side {
            Color::White => self.black,
            Color::Black => self.white,
        })
        .iter()
        .fold(0, |bb, pice_bb| bb | pice_bb)
    }

    pub fn us_bitmap(&self) -> u64 {
        self.get_side_bb(self.turn)
    }
    pub fn them_bitmap(&self) -> u64 {
        self.get_side_bb(self.not_turn())
    }

    pub fn get_them_bb_array(&self) -> [u64; 7] {
        match self.turn {
            Color::White => self.black,
            Color::Black => self.white,
        }
    }

    pub fn get_turns_bb_array(&self) -> [u64; 7] {
        match self.turn {
            Color::Black => self.black,
            Color::White => self.white,
        }
    }

    pub fn count_available_moves(&self) -> u32 {
        let targets = self.get_available_targets(self.turn);
        targets.iter().fold(0, |count, t| count + t.count_ones())
    }

    pub fn count_available_moves_at_index(&self, i: u32, piece: &Piece) -> u32 {
        self.get_available_moves_at_index(i, piece).count_ones()
    }

    pub fn index_to_mask(i: u32) -> u64 {
        1 << i
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

    fn push_unchecked_move(&mut self, m: Move) {
        self.make_unchecked_move(m.from as u8, m.target as u8, m.piece);
        if m.captures.is_some() {
            if m.piece.class == P::Pawn
                && self.en_passant_target.is_some()
                && m.target == self.en_passant_target.unwrap() as u32
            {
                println!("EN KDFAJDFKAJDLKFAD");
                let actual_capture_index = match m.piece.color {
                    Color::White => Direction::Down.value() + m.target as i64,
                    Color::Black => Direction::Up.value() + m.target as i64, // Color::White => Direction::Up(self.white[0])
                };
                self.unplace(m.captures.unwrap(), actual_capture_index as u8);
            } else {
                self.unplace(m.captures.unwrap(), m.target as u8);
            }
        }

        if (m.piece.class == P::Pawn) && m.from.abs_diff(m.target) == 16 {
            println!("en passant sq");

            // let i = &((1 as u64) << (m.target);
            let en_passant_index = match m.piece.color {
                Color::White => Direction::Down.value() + m.target as i64,
                Color::Black => Direction::Up.value() + m.target as i64, // Color::White => Direction::Up(self.white[0])
            };

            self.en_passant_target = Some(en_passant_index as usize);

            // self.place(
            //     Piece {
            //         class: P::Preview,
            //         color: Color::White,
            //     },
            //     en_passant_index as u8,
            // );

            // println!("{:64b}", en_passant_index);
            // println!("{}", en_passant_index);
        } else {
            self.en_passant_target = None;
        }

        if (m.piece.class == P::King) {
            let (cr_from, cr_to) = match (m.piece.color, m.target) {
                // white king side castling
                (Color::White, 62) => {
                    self.white_cr.king = false;
                    (63, 63 + Direction::Left.value() + Direction::Left.value())
                }
                // white queen side castling
                (Color::White, 58) => {
                    self.white_cr.queen = false;
                    (58, 58 + Direction::Right.value() + Direction::Right.value())
                }
                // black king side castling
                (Color::Black, 6) => {
                    self.black_cr.king = false;
                    (7, 7 + Direction::Left.value() + Direction::Left.value())
                }
                // black queen side castling
                (Color::Black, 1) => {
                    self.black_cr.queen = false;
                    (0, 0 + Direction::Right.value() + Direction::Right.value())
                }
                _ => (-1, -1),
            };

            if cr_from > -1 && cr_to > -1 {
                let rook = Piece {
                    color: m.piece.color,
                    class: P::Rook,
                };
                self.make_unchecked_move(cr_from as u8, cr_to as u8, rook);
            }
        }

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

        if m.captures.is_some() && m.captures.unwrap().class == P::Rook {
            match (m.captures.unwrap().color, m.target) {
                // white king side castling
                (Color::White, 63) => self.white_cr.king = false,
                (Color::White, 56) => self.white_cr.queen = false,
                (Color::Black, 7) => self.black_cr.king = false,
                (Color::Black, 0) => self.black_cr.queen = false,
                _ => (),
            };
        }
    }

    pub fn count_attackers_of_square(&self, bb: u64, color: Color) -> u32 {
        println!("bb {:64b}", bb);
        let us_bitmap = self.us_bitmap();
        let them_bitmap = self.them_bitmap();

        let empty = !(us_bitmap | them_bitmap);

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
            empty,
        ) | knight_attacks(self.get_bboard_of_piece(&Piece {
            color,
            class: P::Knight,
        }));

        (bb & op_attacks).count_ones()
    }

    pub fn get_available_moves_of_piece_type(&self, bb: u64, piece: &Piece) -> u64 {
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
                let mut en_passant_bitmap = 0;
                if self.turn == piece.color {
                    en_passant_bitmap = &(1 as u64) << self.en_passant_target.unwrap_or(0);
                }

                let attacks = pawn_attacks(bb, piece.color) & (them_bitmap | en_passant_bitmap); // moves forward

                if piece.color == Color::Black {
                    let first_move_rank = RANK_7;

                    let moves = Direction::Down.shift_once(bb)
                        | (bb & first_move_rank) << 16 & !them_bitmap; // moves forward

                    let fill = occluded_fill(bb, empty, Direction::Down);
                    (fill & moves) | attacks
                } else {
                    let first_move_rank = RANK_2;

                    let moves = (Direction::Up.shift_once(bb) | (bb & first_move_rank) >> 16)
                        & !them_bitmap; // moves forward

                    let fill = occluded_fill(bb, empty, Direction::Up);

                    (fill & moves) | attacks
                }
            }
            P::Queen => queen_attacks(bb, empty) & !us_bitmap,
            P::King => {
                let cr = match piece.color {
                    Color::White => &self.white_cr,
                    Color::Black => &self.black_cr,
                };

                let emtpy_and_not_under_attack = empty & !self.attack_map_of(piece.color.not());
                // let emtpy_and_not_under_attack = !self.attack_map_of(piece.color.not());
                // let emtpy_and_not_under_attack = self.attack_map_of(piece.color.not());

                (king_attacks(bb, emtpy_and_not_under_attack)
                    | king_king_castle(bb, emtpy_and_not_under_attack, cr)
                    | king_queen_castle(bb, emtpy_and_not_under_attack, cr))
                    & !us_bitmap
            }
            P::Rook => rook_attacks(bb, empty) & !us_bitmap,
            P::Bishop => bishop_attacks(bb, empty) & !us_bitmap,
            P::Knight => knight_attacks(bb) & !us_bitmap,
            _ => todo!(),
        }
    }
}
