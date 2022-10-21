<script lang="ts">
	import svelteLogo from './assets/svelte.svg';
	import Counter from './lib/Counter.svelte';
	import BoardElement from './lib/Board.svelte';
  import type { Piece } from './chess/dtos/piece';
	import { Board } from './chess/board';
  import { worker } from './chess/index';


  worker.postMessage({
    hello: "there"
  });
  
	// const initFen = 'r1bqkbnr/1pp2p1p/p1n5/1P1P4/P1p1p1p1/5N2/4PPPP/RNBQKB1R b KQkq - 0 1';
	const initFen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';
	let board = Board.fromFen(initFen);
	console.log('board is', board);

	let _squares = board.squares;
	console.log('squares are', _squares);

  // let squares: [Piece] = JSON.parse(_squares);
</script>

<main>
	<BoardElement {board} />
</main>

<style>
	.logo {
		height: 6em;
		padding: 1.5em;
		will-change: filter;
	}
	.logo:hover {
		filter: drop-shadow(0 0 2em #646cffaa);
	}
	.logo.svelte:hover {
		filter: drop-shadow(0 0 2em #ff3e00aa);
	}
	.read-the-docs {
		color: #888;
	}
</style>
