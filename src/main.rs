mod chess;

// use chess::{Board};
use chess::board::{Board};

fn main() {
    // let board = Board::new();
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut board = Board::from_fen(&fen);

    // board.get_square();
    board.make_move("e2", "e4");
    // board.print();
    board.make_move("d7", "d5");
    // board.print();
    board.make_move("e4", "d5");
    // board.print();
    board.make_move("d8", "d5");
    board.print();

    // dbg!(board.get_available_moves("b1"));
    board.print_available_moves("b1")

    // let a = board.get_square(&"E2".to_string());
    // dbg!(a);
}
