/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
* @param {string} fen
* @returns {string}
*/
export function parse_fen(fen: string): string;
/**
* @param {string} s
* @returns {string}
*/
export function get_squares(s: string): string;
/**
* @param {string} s
* @param {number} depth
* @returns {string}
*/
export function search_good_move(s: string, depth: number): string;
/**
* @param {string} s
* @param {string} _mov
* @returns {string}
*/
export function push_unchecked_move(s: string, _mov: string): string;
/**
* @param {string} s
* @param {number} index
* @param {string} _piece
* @returns {string}
*/
export function get_available_moves_at_index(s: string, index: number, _piece: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number) => void;
  readonly parse_fen: (a: number, b: number, c: number) => void;
  readonly get_squares: (a: number, b: number, c: number) => void;
  readonly search_good_move: (a: number, b: number, c: number, d: number) => void;
  readonly push_unchecked_move: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly get_available_moves_at_index: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
