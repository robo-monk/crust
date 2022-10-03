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
}


impl Board {
  pub fn new() -> Self {
    Board {
      turn: Color::White,
      squares: [Some(Piece::new(P::Pawn, Color::White)); 64]
    }
  }

  pub fn print(&self) -> () {
    println!("{}", self.render_acii());
  }

  pub fn render_acii(&self) -> String {
    let mut out = String::from("");
    for row in 0..8 {
      out += "   ";
      out += &"⎯⎯⎯⎯".repeat(8);
      out += "\n";
      out += &format!("{}  ", 8-row);
      for col in 0..8 {
        out += &format!("⏐ {} ", "K");
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
