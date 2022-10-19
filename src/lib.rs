mod chess;
use chess::bboard::{BBoard, Move};
use std::{thread, time};
// use bitvec::prelude::*;

// use chess::{Board};
use chess::board::Board;
use chess::piece::{Color, Piece, P};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet(name: &str) -> () {
  // return 1
  alert(&format!("Hello, {}!", name));
}
