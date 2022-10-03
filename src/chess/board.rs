#[derive(Debug, Clone, Copy)]
pub enum P {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
  class: P,
  color: Color
}


#[derive(Debug, Clone, Copy)]
pub enum Color {
  Black,
  White
}

#[derive(Debug)]
pub struct Board {
  turn: Color,
  squares: [Option<Piece>; 64]
}


impl Piece {
  pub fn new(class: P, color: Color) -> Self {
    Piece {
      class, color
    }
  }

  pub fn symbol(&self) -> &str {
    match (self.color, self.class) {
       (Color::White, P::Pawn) => "♙",
       (Color::White, P::Knight) => "♘",
       (Color::White, P::Bishop) => "♗",
       (Color::White, P::Rook) => "♖",
       (Color::White, P::Queen) => "♕",
       (Color::White, P::King) => "♔",

       (Color::Black, P::Pawn) => "♟",
       (Color::Black, P::Knight) => "♞",
       (Color::Black, P::Bishop) => "♝",
       (Color::Black, P::Rook) => "♜",
       (Color::Black, P::Queen) => "♛",
       (Color::Black, P::King) => "♚",
    }
  }
}


impl Board {
  pub fn new() -> Self {
    Board {
      turn: Color::White,
      // squares: [Some(Piece::new(P::Pawn, Color::White)); 64]
      squares: [None; 64]
    }
  }

  pub fn print(&self) -> () {
    println!("{}", self.render_acii());
  }

  pub fn get_square(self, square: String) -> Option<Piece> {
    let chs: Vec<char> = square.chars().collect();
    let file = chs[0].to_ascii_lowercase() as u32 - 97; // a is 97 in ascii
    let rank = chs[1].to_digit(10).unwrap_or_else(|| panic!("invalid square")); // 1-8
    let i = ((8-rank)*8 + file) as usize;
    self.squares[i as usize]
  }


  // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
  pub fn from_fen(fen: String) -> Self {
    let mut board = Board::new();
    // let ranks : String = fen.split(" ").collect();
    let fields: Vec<&str> = fen.split(" ").collect();
    // let fields: [&str; 8] = fen.split(" ").collect().try_into();

    let piece_placement_data = fields[0];
    let active_color = fields[1];
    let castling = fields[2];
    let en_passant = fields[3];
    let halfmoves = fields[4]; // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.[9]
    let move_count = fields[5];

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
            _ => panic!("unrecognised char")
          };

          board.squares[index] = Some(Piece::new(class, color));
          println!("laying to i {}: color: {:?} cls: {:?}", index, color, class);
          index += 1;
        }
      })


    });

    board
  }
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
