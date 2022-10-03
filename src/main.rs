mod chess;

// use chess::{Board};
use chess::board::{Board};

fn main() {
    // let board = Board::new();
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let board = Board::from_fen(fen);
    dbg!(&board);
    
    board.print()
}
