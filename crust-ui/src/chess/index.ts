import type { Move } from "./dtos/move";
import type { Piece } from "./dtos/piece";
import { Board } from "./board";

export type { Move, Piece }
export { Board }

// import worker from "./"
export const worker = new Worker(new URL('./worker.js', import.meta.url), {
  type: 'module'
})
