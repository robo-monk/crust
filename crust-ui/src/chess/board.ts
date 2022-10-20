// import init, { greet, parse_fen, get_squares, preview_moves } from "../../crust";
import init, { greet, parse_fen, get_squares } from "../../crust";


await init()

export class Board {
  private s: string
  constructor (s: string) {
    this.s = s;
  }


  // previewMovesOf(sq: number) {
  //   return preview_moves(this.s, sq);
  // }

  get squares() {
    return get_squares(this.s);
  }

  static fromFen(fen: string) {
    return new Board(parse_fen(fen))
  }
}

  // console.log("helloooooo ")
  // greet("Webassambly")
