mod chess;
use chess::bboard::{BBoard, Move, loop_through_indeces};
use std::{thread, time};
// use bitvec::prelude::*;

// use chess::{Board};
use chess::board::Board;
use chess::piece::{Color, Piece, P};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> () {
    // return 1
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn parse_fen(fen: &str) -> String {
    let fen = fen.to_string();
    // let fen = String::from("r1bqkbnr/1pp2p1p/p1n5/1P1P4/P1p1p1p1/5N2/4PPPP/RNBQKB1R b KQkq - 0 1");
    let board = Board::from_fen(&fen);
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

    bboard.serialize()
}


#[wasm_bindgen]
pub fn get_squares(s: &str) -> String {
  let b = BBoard::from_serialization(s);
  let board = b.get_board();
  // let sq = board.squares.map(|p| p.unwrap_or());
  let sq_vec = board.squares.to_vec();
  // String::from("hello")
  serde_json::to_string(&sq_vec).unwrap()
}

#[wasm_bindgen]
pub fn search_good_move(s: &str, depth: i32) -> String {
  let b = BBoard::from_serialization(s);
  let m = b.search_good_move(depth as u32);
  serde_json::to_string(&m).unwrap()
}

// #[wasm_bindgen]
// pub fn make_move(s: &str, _mov: &str) -> String {
//   let mut b = BBoard::from_serialization(s);
//   let mov: Move  = serde_json::from_str(_mov).unwrap();
//   // b.make(&mov);
//   b.serialize()
// }

#[wasm_bindgen]
pub fn push_unchecked_move(s: &str, _mov: &str) -> String {
  let mut b = BBoard::from_serialization(s);
  let mov = serde_json::from_str(_mov).unwrap();
  b.push_unchecked_move(&mov);
  b.serialize()
}


#[wasm_bindgen]
pub fn get_available_moves_at_index(s: &str, index: i32,  _piece: &str) -> String {
  let mut b = BBoard::from_serialization(s);
  let piece: Piece = serde_json::from_str(_piece).unwrap();
  let moves = b.get_available_moves_at_index(index as u32, &piece);
  let mut _moves: Vec<i32> = vec![];
  loop_through_indeces(moves, |i| _moves.push(i as i32));
  serde_json::to_string(&_moves).unwrap()
}
