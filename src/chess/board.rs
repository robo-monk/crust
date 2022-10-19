use std::collections::HashMap;

use super::piece::{Color, Direction, Piece, P};
use serde::{Deserialize, Serialize};
use super::bboard::BBoard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: usize,
    pub target: usize,
		// pub capture: Option<usize>
}

#[derive(Debug, Clone,)]
pub struct Board {
    pub turn: Color,
    pub move_count: u64,
    pub squares: [Option<Piece>; 64],
    pub pieces: HashMap<u8, Piece>,
    pub en_passant: Option<usize>,
    pub white_cr: CastlingRights,
    pub black_cr: CastlingRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastlingRights {
    pub queen: bool,
    pub king: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            turn: Color::White,
            move_count: 0,
            pieces: HashMap::new(),
            // squares: [Some(Piece::new(P::Pawn, Color::White)); 64]
            squares: [None; 64],
            en_passant: None,
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

    pub fn print(&self) -> () {
        println!("{}", self.render_acii());
    }

    pub fn parse_notation(notation: &String) -> Option<usize> {
        if notation == &String::from("-") {
            return None;
        }

        let chs: Vec<char> = notation.chars().collect();
        let file = chs[0].to_ascii_lowercase() as u32 - 97; // a is 97 in ascii
        let rank = chs[1]
            .to_digit(10)
            .unwrap_or_else(|| panic!("invalid square")); // 1-8
        Some(((8 - rank) * 8 + file) as usize)
    }

    pub fn get_all_possible_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for (i, piece) in self.pieces.iter() {
            // moves.iter().chain(self._get_available_moves(i))
            // dbg!(self.squares[i]);
            moves.append(&mut self._get_available_moves(piece, *i as usize));
            // s.get
            // moves.push(s)
        }
        moves
    }

    pub fn get_index(&self, index: usize) -> Option<Piece> {
        self.squares[index]
    }
    pub fn get_square(&self, square: &String) -> Option<Piece> {
        let i = Board::parse_notation(square).unwrap();
        self.squares[i]
    }

    pub fn _set_square(&mut self, index: usize, p: Option<Piece>) -> () {
        self.squares[index] = p;
    }
    pub fn set_square(&mut self, square: &String, p: Option<Piece>) -> () {
        let i = Board::parse_notation(square).unwrap();
        self._set_square(i, p);
    }

    pub fn push_move(&mut self, m: &Move) -> () {
        self._make_move(m.from, m.target);
    }

    // pub fn _make_move(&mut self, from: &String, to: &String) -> () {
    pub fn _make_move(&mut self, from: usize, to: usize) -> () {
        // let piece = self.get_square(from);
        let piece = self.squares[from];
        let target_piece = self.squares[to];

        if piece.is_none() {
            panic!("invalid move")
            // return
        }

        // special rules
        match piece.unwrap().class {
            P::Pawn => {
                // en passant
                if self.en_passant.is_some() && to == self.en_passant.unwrap() {
                    match piece.unwrap().color {
                        Color::White => {
                            self._set_square(
                                (to + Direction::Down)
                                    .unwrap_or_else(|| panic!("invalid en passant move")),
                                None,
                            );
                            // self.white_cr.king = false;
                            // self.white_cr.queen = false;
                        }
                        Color::Black => {
                            self.black_cr.king = false;
                            self.black_cr.queen = false;
                        }
                    }
                }
            }
            P::King => {
                // if king moved no castling rights
                match piece.unwrap().color {
                    Color::White => {
                        self.white_cr.king = false;
                        self.white_cr.queen = false;

                        // king side castle
                        if to == 6 {
                            self._set_square(5, self.get_index(7));
                            self._set_square(7, None);
                        }

                        if to == 2 {
                            self._set_square(3, self.get_index(0));
                            self._set_square(0, None);
                        }
                    }
                    Color::Black => {
                        self.black_cr.king = false;
                        self.black_cr.queen = false;
                    }
                }
            }
            P::Rook => {
                // if rook moved no castling rights for that side
                if from == 0 {
                    self.black_cr.queen = false
                }
                if from == 7 {
                    self.black_cr.king = false
                }
                if from == 56 {
                    self.white_cr.queen = false
                }
                if from == 63 {
                    self.white_cr.king = false
                }
            }
            _ => (),
        }

        if target_piece.is_some() {
            match target_piece.unwrap().class {
                P::Rook => {
                    // if rook that got captured no castling rights for that side
                    if from == 0 {
                        self.black_cr.queen = false
                    }
                    if from == 7 {
                        self.black_cr.king = false
                    }
                    if from == 56 {
                        self.white_cr.queen = false
                    }
                    if from == 63 {
                        self.white_cr.king = false
                    }
                }
                _ => (),
            }
        }

        self._set_square(to, piece);
        self._set_square(from, None);

        self.turn = if self.turn == Color::Black {
            Color::White
        } else {
            Color::Black
        };

        self.move_count += 1;
    }

    pub fn make_move(&mut self, from: &str, to: &str) -> () {
        let _from = Board::parse_notation(&from.to_string()).unwrap();
        let _to = Board::parse_notation(&to.to_string()).unwrap();

        self._make_move(_from, _to);
    }

    // pub fn get_available_moves(&self, notation: &str) -> Vec<Move> {
    pub fn get_available_moves(&self, notation: &str) -> Vec<Move> {
        let index = Board::parse_notation(&notation.to_string()).unwrap();
        self._get_available_moves(self.pieces.get(&(index as u8)).unwrap(), index)
    }

    pub fn count_ply_moves(&self, depth: usize) -> usize {
        if depth <= 0 {
            return 1;
        }

        let moves = self.get_all_possible_moves();
        // let mut move_count = moves.len();
        let mut move_count = 0;

        for m in &moves {
            let mut _board = self.clone();
            _board.push_move(m);
            move_count += self.count_ply_moves(depth - 1);
        }

        move_count
    }

    // pub fn get_available_moves(&self, notation: &str) -> Vec<Move> {
    pub fn _get_available_moves(&self, piece: &Piece, index: usize) -> Vec<Move> {
        // let index = Board::parse_notation(&notation.to_string()).unwrap();
        // let piece = self.squares[index].unwrap();
        // let piece = self.get_square(&notation.to_string()).unwrap_or_else(|| panic!("can't get moves of not a piece"));

        if piece.color != self.turn {
            return vec![];
        }

        let paths = piece.get_paths(index, &self);

        let mut square_targets: Vec<usize> = vec![];

        paths.iter().for_each(|path| {
            let mut target = Some(index);

            if piece.is_sliding() {
                // println!("path is {path:?}");
                let direction = *path.get(0).unwrap();

                loop {
                    target = target.unwrap() + direction;

                    // if target is out of bounds, exit
                    if target.is_none() {
                        break;
                    }

                    let _target_piece = self.squares[target.unwrap()];

                    // if target has a piece, make some checks
                    if _target_piece.is_some() {
                        let target_piece = _target_piece.unwrap();

                        // if piece can be capture, add the target to square targets and exit
                        if piece.can_capture(&target_piece) {
                            square_targets.push(target.unwrap());
                        }

                        break;
                    } else {
                        square_targets.push(target.unwrap());
                    }
                }
            } else {
                for (i, direction) in path.iter().enumerate() {
                    target = target.unwrap() + *direction;
                    if target.is_none() {
                        break;
                    }

                    let _target_piece = self.squares[target.unwrap()];

                    if _target_piece.is_some() {
                        let target_piece = _target_piece.unwrap();

                        // if piece can't land (if final step ) or pass through the next step piece, burn the path

                        if !(*piece >> target_piece) || i == path.len() - 1 {
                            if *piece ^ target_piece {
                                // println!("CAN EAT> {:?}", target_piece);
                                square_targets.push(target.unwrap());
                            } else {
                                target = None;
                            }
                            break;
                        }
                    }
                }

                if target.is_some() {
                    square_targets.push(target.unwrap());
                };
            }
        });

        square_targets
            .iter()
            .map(|target| Move {
                from: index,
                target: *target,
            })
            .collect()
    }

    pub fn print_available_moves(&mut self, notation: &str) -> () {
        let moves = self.get_available_moves(notation);
        // let &mut board = self.clone_into(&mut Board);
        // let &mut board = self.clonle_into(&mut Board);
        // let mut clone = self.clone();
        let mut board: &mut Board = &mut self.clone();
        // let board: &mut Board = &clone;

        moves.iter().for_each(|t| {
            board._set_square(t.target, Some(Piece::new(P::Preview, Color::White)));
        });
        board.print()
    }

    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn from_fen(fen: &String) -> Self {
        // let ranks : String = fen.split(" ").collect();
        let fields: Vec<&str> = fen.split(" ").collect();
        // let fields: [&str; 8] = fen.split(" ").collect().try_into();

        let piece_placement_data = fields[0];
        let active_color = fields[1]; // w or b
        let castling = fields[2];
        let en_passant = fields[3];
        let halfmoves = fields[4]; // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.[9]
        let move_count = fields[5];

        // let mut board = Board {
        let mut board = Board::new();

        board.turn = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("invalid fen"),
        };

        board.move_count = move_count.parse::<u64>().unwrap();
        board.en_passant = Board::parse_notation(&en_passant.to_string());

				// parse castling rights
        castling.chars().for_each(|c| {
            let color = if c.is_lowercase() {
                Color::Black
            } else {
                Color::White
            };

            let side: P = match c.to_lowercase().to_string().as_str() {
                "k" => P::King,
                "q" => P::Queen,
                _ => panic!("invalid fen when parsing castling rights"),
            };

						match (color, side) {
							(Color::White, P::King) => board.white_cr.king = true,
							(Color::White, P::Queen) => board.white_cr.queen = true,
							(Color::Black, P::King) => board.black_cr.king = true,
							(Color::Black, P::Queen) => board.black_cr.queen = true,
							_ => panic!("invalid fen when parsing cr")
						}
        });
        // parse placement data
        let mut index: usize = 0;
        piece_placement_data.split("/").for_each(|rank| {
            rank.chars().for_each(|p| {
                if p.is_ascii_digit() {
                    print!("ascii digit incr from {}", index);
                    index += p.to_digit(10).unwrap_or_else(|| panic!("expected number")) as usize;
                    println!(" to {}", index);
                } else {
                    let color = if p.is_lowercase() {
                        Color::Black
                    } else {
                        Color::White
                    };
                    let class: P = match p.to_lowercase().to_string().as_str() {
                        "p" => P::Pawn,
                        "k" => P::King,
                        "q" => P::Queen,
                        "b" => P::Bishop,
                        "n" => P::Knight,
                        "r" => P::Rook,
                        _ => panic!("invalid fen"),
                    };

                    let piece = Piece::new(class, color);
                    board.squares[index] = Some(piece);
                    board.pieces.insert(index as u8, piece);
                    println!("laying to i {}: color: {:?} cls: {:?}", index, color, class);
                    index += 1;
                }
            })
        });

        board
    }

    // cancer
    pub fn render_acii(&self) -> String {
        let mut out = String::from("");
        for rank in 0..8 {
            out += "   ";
            out += &"⎯⎯⎯⎯".repeat(8);
            out += "\n";
            out += &format!("{}  ", 8 - rank);
            for file in 0..8 {
                let i = 8 * rank + file;
                if self.squares[i].is_some() {
                    out += &format!("⏐ {} ", self.squares[i].unwrap().symbol());
                } else {
                    out += &format!("⏐ {} ", " ");
                }
            }
            out += "⏐\n";
        }

        out += "   ";
        out += &"⎯⎯⎯⎯".repeat(8);
        out += "\n";
        out += "   ";
        out += "  a   b   c   d   e   f   g   h";
        out += "\n";

        out
        // self.squares.iter().for_each(|square| {
        // });
    }
}
