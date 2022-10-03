mod chess;

// use chess::{Board};
use chess::board::{Board};

fn main() {
    // let board = Board::new();
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut board = Board::from_fen(&fen);
    dbg!(&board);

    // board.get_square();
    board.make_move("e2", "e4");
    board.print();

    // let a = board.get_square(&"E2".to_string());
    // dbg!(a);
}
