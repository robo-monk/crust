import type { Piece } from "./piece";

export interface Move {
  from: number,
  target: number,
  piece: Piece,
  captures: Piece,
};
