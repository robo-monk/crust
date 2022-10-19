mod chess;
use chess::bboard::{BBoard, Move};
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
