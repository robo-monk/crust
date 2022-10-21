import init, { greet, parse_fen, get_squares, search_good_move, get_available_moves_at_index, push_unchecked_move } from "../../crust";
await init()

const crust = { greet, parse_fen, get_squares, search_good_move, get_available_moves_at_index, push_unchecked_move };

// @ts-ignore
onmessage = async (message) => {
  const {
    id, fn, params
  } = message.data;

  const res = await crust[fn](...params);

  postMessage({
    id,
    res
  })
}
