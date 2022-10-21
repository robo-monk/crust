// import init, { greet, parse_fen, get_squares, preview_moves } from "../../crust";
import init, { greet, parse_fen, get_squares, search_good_move, get_available_moves_at_index, push_unchecked_move } from "../../crust";
import type { Piece } from "./dtos/piece";
import type { Move } from "./dtos/move";
import { nanoid } from "nanoid";


await init()

export class Board {
  private s: string
  private worker: Worker;
  private cbs: Map<string, Function>
  constructor(s: string) {
    this.s = s;
    this.cbs = new Map()

    this.worker = new Worker(new URL('./worker.ts', import.meta.url), {
      type: 'module'
    })

    this.worker.addEventListener("message", (ev: MessageEvent) => {
      if (this.cbs.has(ev.data.id)) {
        const cb = this.cbs.get(ev.data.id)
        cb(ev.data.res);
      }
    });

  }

  // previewMovesOf(sq: number) {
  //   return preview_moves(this.s, sq);
  // }

  getAvailableMoveOfSquare(piece: Piece, square: number) {

  }

  async #exec(fn: string, ...params): Promise<any> {
    const id = nanoid();

    this.worker.postMessage({
      id, fn, params
    });

    return new Promise(resolve => {
      this.cbs.set(id, (res) => {
        resolve(res);
        this.cbs.delete(id);
      })
    })
  }

  get squares() {
    return JSON.parse(get_squares(this.s));
  }
  

  static fromFen(fen: string) {
    return new Board(parse_fen(fen))
  }

  async searchGoodMove(depth: number) {
    let res = await this.#exec("search_good_move", this.s, depth)
    return JSON.parse(res)
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
