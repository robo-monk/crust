use super::piece::{P, Piece, Color, Direction};

pub struct Move {
  from: usize,
  to: usize
}

#[derive(Debug)]
pub struct Board {
  turn: Color,
  move_count: u64,
  squares: [Option<Piece>; 64],
  en_passant: Option<usize>,
}

impl Board {
  pub fn new() -> Self {
    Board {
      turn: Color::White,
      move_count: 0,
      // squares: [Some(Piece::new(P::Pawn, Color::White)); 64]
      squares: [None; 64],
      en_passant: None,
    }
  }

  pub fn print(&self) -> () {
    println!("{}", self.render_acii());
  }

  pub fn parse_notation(notation: &String) -> Option<usize> {
    if notation == &String::from("-") { return None }

    let chs: Vec<char> = notation.chars().collect();
    let file = chs[0].to_ascii_lowercase() as u32 - 97; // a is 97 in ascii
    let rank = chs[1].to_digit(10).unwrap_or_else(|| panic!("invalid square")); // 1-8
   Some(((8-rank)*8 + file) as usize)
  }

  pub fn get_square(&self, square: &String) -> Option<Piece> {
    let i = Board::parse_notation(square).unwrap();
    self.squares[i]
  }

  pub fn set_square(&mut self, square: &String, p: Option<Piece>) -> () {
    let i = Board::parse_notation(square).unwrap();
    self.squares[i] = p;
  }

  pub fn _make_move(&mut self, from: &String, to: &String) -> () {
    let piece = self.get_square(from);

    if piece.is_none() {
      panic!("invalid move")
    }

    self.set_square(to, piece);
    self.set_square(from, None);

    self.turn = if self.turn == Color::Black {
      Color::White
    } else {
      Color::Black
    };

    self.move_count += 1;
  }

  pub fn make_move(&mut self, from: &str, to: &str) -> () {
    self._make_move(&from.to_string(), &to.to_string());
  }

  // pub fn get_available_moves(&self, notation: &str) -> Vec<Move> {
  pub fn get_available_moves(&self, notation: &str) -> Vec<u64> {
    let index = Board::parse_notation(&notation.to_string()).unwrap();
    let piece = self.squares[index].unwrap();
    // let piece = self.get_square(&notation.to_string()).unwrap_or_else(|| panic!("can't get moves of not a piece"));

    if piece.color != self.turn {
      return vec![];
    }

    let diff = match piece.class {
      P::Pawn => {
        todo!()
      },
      P::Knight => {
        vec![
          Direction::Up + Direction::UpLeft,
          Direction::Up + Direction::UpRight,

          Direction::Down + Direction::DownLeft,
          Direction::Down + Direction::DownRight,

          Direction::Left + Direction::UpLeft,
          Direction::Left + Direction::DownLeft,

          Direction::Right + Direction::DownRight,
          Direction::Right + Direction::DownRight,
        ]
      }
      _ => panic!("invalid piece?")
    };


    diff.iter()
      .map(|d| index as i64 + d)
      .filter(|t| t >= &0 && t < &64)
      .map(|t| t.abs() as u64)
      .collect::<Vec<u64>>()
  }

  // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
  pub fn from_fen(fen: &String) -> Self {
    // let ranks : String = fen.split(" ").collect();
    let fields: Vec<&str> = fen.split(" ").collect();
    // let fields: [&str; 8] = fen.split(" ").collect().try_into();

    let piece_placement_data = fields[0];
    let active_color = fields[1]; // w or b
    let castling = fields[2];
    let en_passant = fields[3];
    let halfmoves = fields[4]; // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.[9]
    let move_count = fields[5];


    let mut board = Board {
      turn:  match active_color {
        "w" => Color::White,
        "b" => Color::Black,
        _ => panic!("invalid fen")
      },
      move_count: move_count.parse::<u64>().unwrap(),
      squares: [None; 64],
      en_passant: Board::parse_notation(&en_passant.to_string())
    };

    // parse placement data
    let mut index: usize = 0;
    piece_placement_data.split("/").for_each(|rank| {
      rank.chars().for_each(|p| {
        if p.is_ascii_digit() {
          print!("ascii digit incr from {}", index);
          index += p.to_digit(10).unwrap_or_else(|| panic!("expected number")) as usize;
          println!(" to {}", index);
        } else {
          let color =  if p.is_lowercase() { Color::Black } else { Color::White };
          let class: P = match p.to_lowercase().to_string().as_str() {
            "p" => P::Pawn,
            "k" => P::King,
            "q" => P::Queen,
            "b" => P::Bishop,
            "n" => P::Knight,
            "r" => P::Rook,
            _ => panic!("invalid fen")
          };

          board.squares[index] = Some(Piece::new(class, color));
          println!("laying to i {}: color: {:?} cls: {:?}", index, color, class);
          index += 1;
        }
      })
    });

    board
  }


  // cancer
  pub fn render_acii(&self) -> String {
    let mut out = String::from("");
    for rank in 0..8 {
      out += "   ";
      out += &"⎯⎯⎯⎯".repeat(8);
      out += "\n";
      out += &format!("{}  ", 8 - rank);
      for file in 0..8 {
        let i = 8*rank + file;
        if self.squares[i].is_some() {
          out += &format!("⏐ {} ", self.squares[i].unwrap().symbol());
        } else {
          out += &format!("⏐ {} ", " ");
        }
      }
      out += "⏐\n";
    }

    out += "   ";
    out += &"⎯⎯⎯⎯".repeat(8);
    out += "\n";
    out += "   ";
    out += "  a   b   c   d   e   f   g   h";
    out += "\n";

    out
    // self.squares.iter().for_each(|square| {
    // });
  }

  
}
