mod chess;

// use chess::{Board};
use chess::board::Board;

fn main() {
    // let board = Board::new();
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut board = Board::from_fen(&fen);

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
    let ply = 5;
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
