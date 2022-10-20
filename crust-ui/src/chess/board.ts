// import init, { greet, parse_fen, get_squares, preview_moves } from "../../crust";
import init, { greet, parse_fen, get_squares, search_good_move, get_available_moves_at_index } from "../../crust";
import type { Piece } from "./dtos/piece";


await init()

export class Board {
  private s: string
  constructor (s: string) {
    this.s = s;
  }


  // previewMovesOf(sq: number) {
  //   return preview_moves(this.s, sq);
  // }

  getAvailableMoveOfSquare(piece: Piece, square: number) {

  }
  get squares() {
    return JSON.parse(get_squares(this.s));
  }

  static fromFen(fen: string) {
    return new Board(parse_fen(fen))
  }

  // searchGoodMove(sq: number) {
  //   return _search_good_move()
  // }

  getAvailableMovesAtIndex(index: number, piece: Piece) {
    JSON.parse(get_available_moves_at_index(this.s, index, JSON.stringify(piece)))
  }
}

  // console.log("helloooooo ")
  // greet("Webassambly")
