import init, { greet, parse_fen, get_squares } from "../../crust";

const initFen = "r1bqkbnr/1pp2p1p/p1n5/1P1P4/P1p1p1p1/5N2/4PPPP/RNBQKB1R b KQkq - 0 1"

await init()

export class Board {
  private s: string
  constructor (s: string) {
    this.s = s;
  }

  get squares() {
    return get_squares(this.s);
  }

  static fromFen(fen: string) {
    return new Board(parse_fen(fen))
  }
}

  // console.log("helloooooo ")
let board = Board.fromFen(initFen);
console.log('board is', board);

let squares = board.squares;
console.log('squares are', squares);

  // greet("Webassambly")
