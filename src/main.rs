mod chess;

// use chess::{Board};
use chess::board::{Board};

fn main() {
    let board = Board::new();
    dbg!(&board);
    board.print()
}
