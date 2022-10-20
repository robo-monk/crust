// import init, { greet, parse_fen, get_squares, preview_moves } from "../../crust";
import init, { greet, parse_fen, get_squares, search_good_move, get_available_moves_at_index, push_unchecked_move } from "../../crust";
import type { Piece } from "./dtos/piece";
import type { Move } from "./dtos/move";


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

  async searchGoodMove(depth: number) {
    let m = search_good_move(this.s, depth)
    return JSON.parse(m);
  }

  pushUncheckedMove(move: Move) {
    this.s = push_unchecked_move(this.s, JSON.stringify(move))
  }

  getAvailableMovesAtIndex(index: number, piece: Piece): Set<number> {
    let ii = JSON.parse(get_available_moves_at_index(this.s, index, JSON.stringify(piece)))
    return new Set(ii)
  }
}

  // console.log("helloooooo ")
  // greet("Webassambly")
