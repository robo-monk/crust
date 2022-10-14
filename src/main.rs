mod chess;
use std::{thread, time};
use chess::bboard::BBoard;
// use bitvec::prelude::*;


// use chess::{Board};
use chess::piece::{Piece, P, Color};
use chess::board::Board;

fn main() {
    // let board = Board::new();
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let board = Board::from_fen(&fen);
    let mut bboard = BBoard::new();

    bboard.white_cr = board.white_cr;
    bboard.black_cr = board.black_cr;
    bboard.en_passant_target = board.en_passant;

    for (i, sq) in board.squares.iter().enumerate() {
        if sq.is_none() { continue }
        let piece = sq.unwrap();
        dbg!(i, piece);

        bboard.place(piece, i as u8);
    }

    bboard.pprint();

    // bboard.make_unchecked_move()
    // bboard.register_unchecked_move("d2", "d3", Piece::new(P::Pawn, Color::White));
    // bboard.register_unchecked_move("e7", "e4", Piece::new(P::Pawn, Color::Black));
    // bboard.register_unchecked_move("g7", "g5", Piece::new(P::Pawn, Color::Black));
    // bboard.register_unchecked_move("h7", "h6", Piece::new(P::Pawn, Color::Black));

    // bboard.register_unchecked_move("d8", "b4", Piece::new(P::Queen, Color::Black));
    // bboard.register_unchecked_move("d8", "h3", Piece::new(P::Queen, Color::Black));

    // bboard.register_unchecked_move("d1", "d5", Piece::new(P::Queen, Color::White));
    // bboard.register_unchecked_move("d1", "d2", Piece::new(P::Queen, Color::White));
    // bboard.register_unchecked_move("d1", "d8", Piece::new(P::Queen, Color::White));
    // bboard.register_unchecked_move("a2", "a3", Piece::new(P::Pawn, Color::White));

    // let W_PAWN = &Piece::new(P::Pawn, Color::White);
    // let B_QUEEN = &Piece::new(P::Queen, Color::Black);
    // bboard.preview_moves_of("a2", W_PAWN);
    // bboard._move("b2", "b4", P::Pawn);
    // bboard._move("a7", "a5", P::Pawn);
    // bboard._move("c2", "c4", P::Pawn);
    // bboard._move("a7", "a5", P::Pawn);
    // bboard._move("c4", "c5", P::Pawn);
    // bboard._move("d7", "d5", P::Pawn);

    // bboard.clone().preview_moves_of("c5", P::Pawn);
    // bboard._capture("c5", "d6", P::Pawn, P::Pawn);
    // bboard.clone().preview_moves_of("h7", P::Pawn);

    // bboard._move("g1", "f3", P::Knight);
    // bboard._move("b8", "c6", P::Knight);

    // bboard._move("g2", "g3", P::Pawn);
    // bboard._move("e7", "e6", P::Pawn);
    // bboard._move("f1", "g2", P::Bishop);

    // // bboard._move("d7", "d5", P::Pawn);
    // // bboard._move("g8", "f6", P::Knight);

    // bboard._move("d1", "d4", P::Queen);
    // bboard._move("h7", "h6", P::Pawn);

    // bboard._move("c1", "a3", P::Bishop);
    // bboard._move("h6", "h5", P::Pawn);
    // bboard._move("b1", "c3", P::Knight);
    // bboard._move("h5", "h4", P::Pawn);
    // bboard._move("h2", "h4", P::Pawn);
    // bboard._move("g8", "f6", P::Knight);
    // bboard.pprint();
    bboard.clone().preview_moves_of("a1", P::Rook, Color::White);
    bboard.pprint();

    // bboard._move("e5", "e6", P::Pawn);
    // bboard._move("b7", "b6", P::Pawn);
    // bboard.clone().preview_moves_of("e1", P::King);
    // bboard._move("d2", "d4", P::Pawn);
    // bboard._move("c8", "a6", P::Bishop);
    // bboard.clone().preview_moves_of("e1", P::King);
    // bboard.count_attackers_of_square("e4");
    // bboard._move("f8", "e7", P::Bishop);
    // bboard.clone().preview_attackers();
    // bboard.clone().preview_moves_of("e1", P::King);

    // bboard.clone().preview_moves_of("a6", P::Bishop);
    let ply = 3;

    let m = bboard.count_ply_moves(ply);
    println!("ply({ply}) -> {m}");
    // bboard.pprint();

    // println!("sq g1 {}", BBoard::parse_sq("a8"));
    // // println!("sq h1 {}", BBoard::parse_sq("h1"));
    // loop {
    //     thread::sleep(time::Duration::from_millis(100));
    //     bboard.make_random_move();
    //     bboard.pprint();
    // }


    // bboard.pprint();
    // bboard.get_available_captures(Piece::new(P::Pawn, Color::White));
    // bboard.preview_moves(&Piece::new(P::Pawn, Color::White));
    // bboard.clone().preview_moves(&Piece::new(P::Queen, Color::White));

    // bboard.clone().preview_moves(&Piece::new(P::Queen, Color::Black));
    // bboard.pprint();

    // bboard.clone().preview_moves(&Piece::new(P::Pawn, Color::White));
    // bboard.clone().preview_moves(&Piece::new(P::Queen, Color::White));

    // bboard.register_unchecked_move("d1", "d2", Piece::new(P::Queen, Color::White));

    // bboard.pprint();
    // bboard.place(Piece::new(P::Pawn, Color::White), 4);
    return;

    // board.get_square();
    // board.make_move("e2", "e4");
    // board.make_move("d7", "d5");
    // board.make_move("e4", "d5");
    // board.make_move("d8", "d5");
    // board.make_move("a2", "a3");
    // // // board.print();

    // // // dbg!(board.get_available_moves("b1"));
    // board.print_available_moves("b8");
    // board.make_move("b8", "d7");
    // // // board.print_available_moves("b1"); // knight moves
    // board.make_move("b1", "c3");
    // // // board.print_available_moves("d7"); // knight moves

    // board.make_move("g8", "f6");
    // board.make_move("d1", "e2");
    // board.print_available_moves("d5"); // queen moves

    // board.make_move("e7", "e6");
    // board.print_available_moves("e2"); // queen moves
    // board.make_move("e2", "e5");
    // board.print_available_moves("b7"); // pawn moves
    // board.make_move("b7", "b5");
    // board.make_move("d2", "d4");
    // board.print_available_moves("b5"); // pawn moves
    // board.make_move("b5", "b4");
    // board.print_available_moves("f1"); // bishiop moves
    // board.make_move("f1", "c4");
    // board.make_move("f8", "a4");
    // board.print_available_moves("c4"); // bishiop moves

    // let moves = board.get_all_possible_moves();

    // for m in &moves {
    //     let mut _board = board.clone();
    //     _board.push_move(m);
    // }

    // println!("count is {c}");

    // moves.len()
    // for m in moves {
    // board._make_move()
    // }
    // println!("total moves possible {:?} [len: {:?}]", moves, moves.len());
    // board.make_move("b5", "b4");
    // board.print_available_moves("b1");

    // board.print_available_moves("g1")

    // let a = board.get_square(&"E2".to_string());
    // dbg!(a);
    let ply = 2;
    let count = board.count_ply_moves(ply);
    println!("count_ply_moves({ply}) -> {count}");
}

// #[cfg(test)]
// mod tests {

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
#[test]
fn rules_are_right() {
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut board = Board::from_fen(&fen);
    // let plys_count = hashmap![1 => 20, 2 => 400, 3 => 8902, 4 => 197281, 5 => 4865609, 6 => 119060324 ];
    let plys_count = hashmap![1 => 20, 2 => 400, 3 => 8902, 4 => 197281, 5 => 4865609 ];
    // let plys_count = hashmap![1 => 20, 2 => 400, 3 => 8902, 4 => 197281];

    for (ply, expected_count) in plys_count.iter() {
        let count = board.count_ply_moves(*ply);
        assert_eq!(count, *expected_count as usize, "ply {ply} should be {expected_count} but was {count}");
    }
}
// }
