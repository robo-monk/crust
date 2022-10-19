import init, { greet, parse_fen } from "../../crust";

const initFen = "r1bqkbnr/1pp2p1p/p1n5/1P1P4/P1p1p1p1/5N2/4PPPP/RNBQKB1R b KQkq - 0 1"

init().then(() => {
  // console.log("helloooooo ")
  let board = parse_fen(initFen);
  console.log('board is', board);
  // greet("Webassambly")

})


export class Board {
}
