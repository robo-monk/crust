mod chess;
use chess::bboard::{BBoard, Move};
use std::{thread, time};
// use bitvec::prelude::*;

// use chess::{Board};
use chess::board::Board;
use chess::piece::{Color, Piece, P};

fn main() {
    // let board = Board::new();
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
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

    bboard.pprint();

    let ply = 4;

    let m = bboard.count_ply_moves(ply);
    println!("ply({ply}) -> {m}");

    // bboard.preview_moves(&Piece { class: P::Pawn, color: Color::Black });
    println!("starting agent at (depth: {ply})");
    // bboard.make_good_move(ply);
    bboard.pprint();


    // println!("sq g1 {}", BBoard::parse_sq("a8"));
    // println!("sq h1 {}", BBoard::parse_sq("h1"));

    // loop
    //     thread::sleep(time::Duration::from_millis(500));
    //     // bboard.make_random_move();
    //     // bboard.make_good_move(ply);
    //     bboard.make_good_move(4);
    //     bboard.pprint();
    // }

    loop {

        bboard.make_good_move(ply);
        bboard.pprint();

        let mut line = String::new();
        println!("Enter your move (ex. e2 e4, e4xd5): ");
        std::io::stdin().read_line(&mut line).unwrap();

        if line.contains("x") {
            let contents: Vec<&str> = line.split("x").collect();
            let from = BBoard::parse_sq(contents.get(0).unwrap()) as u32;
            let target = BBoard::parse_sq(contents.get(1).unwrap()) as u32;
            let piece = bboard.find_piece_at_index(from, bboard.turn);
            let captures = bboard.find_piece_at_index(target, bboard.turn.not());

            bboard.push_unchecked_move(&Move {
                target,
                from,
                piece,
                captures: Some(captures),
            });
        } else {
            let contents: Vec<&str> = line.split(" ").collect();
            let from = BBoard::parse_sq(contents.get(0).unwrap()) as u32;
            let target = BBoard::parse_sq(contents.get(1).unwrap()) as u32;
            let piece = bboard.find_piece_at_index(from, bboard.turn);

            bboard.push_unchecked_move(&Move {
                target,
                from,
                piece,
                captures: None,
            });
        }

        // let from_sq = line.split("")
        // println!("Hello , {}", line);
        // println!("no of bytes read , {}", b1);
    }

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
        assert_eq!(
            count, *expected_count as usize,
            "ply {ply} should be {expected_count} but was {count}"
        );
    }
}
// }
