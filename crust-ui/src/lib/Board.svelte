<script lang="ts">
	import { onMount } from 'svelte';
	import type { Piece } from '../chess/dtos/piece';
	import type { Move } from '../chess/dtos/move';
	import PieceElement from './Piece.svelte';
	import { Droppable } from '@shopify/draggable';
	import type { Board } from '../chess/board';

	export let board: Board;

	let squares: [Piece];

	// $: {
	// if (board) {
	//   console.log('board sq', board.squares)
	// 	squares = JSON.parse(board.squares);
	// }
	// }
	const getWhiteOrBlackSq = (i: number) => {
		const getRank = (i) => Math.floor(i / 8);

		return (i + getRank(i)) % 2 == 0;
	};

	const MOVE: Move = {
		from: null,
		target: null,
		piece: null,
		captures: null,
	};

	let move = MOVE;
	let availableMoves = new Set();

	onMount(() => {
		const draggable = new Droppable(document.querySelectorAll('.board'), {
			draggable: '.piece',
			dropzone: '.square',
		});

		const getIndex = (e) => parseInt(e?.data?.dropzone?.dataset.i);

		draggable.on('droppable:start', (e) => {
			move = MOVE;

			const from = getIndex(e);
			const piece = board.squares[from];

			move.from = from;
			move.piece = piece;

			availableMoves = board.getAvailableMovesAtIndex(from, piece);
			console.log('moves are', availableMoves);
		});

		draggable.on('droppable:dropped', (e) => {
      const _pieces = e.data.dropzone.querySelectorAll(".piece")
      // console.log(_pieces, "<<<<<<<<<<<<<<<<")
      // if (_pieces.length) {
      //   Array.from(_pieces).forEach(_piece => _piece.style.display = 'none')
      // }
      // console.log(e);
      let i = getIndex(e)
      // console.log('i is', i, availableMoves);
      if (!availableMoves.has(i)) e.cancel()
      // e.stopPropagation()
    })

		draggable.on('droppable:stop', async (e) => {
      availableMoves = new Set()
			const target = getIndex(e);

      if (target === move.from) {
         return console.log("cancel move");
      }

			const capturedPiece = board.squares[target];

			move.target = target;

			if (capturedPiece) {
				move.captures = capturedPiece;
			}

      // make move

			console.log('move is', move);
      board.pushUncheckedMove(move)

      let goodMove= await board.searchGoodMove(4);
      // let goodMove= await board.searchGoodMove(3);
      console.log("good move is", goodMove)
      board.pushUncheckedMove(goodMove)

      board = board
		});
	});
</script>

<div class="board">
	{#if board}
		{#each board.squares as sq, i}
			<div class="square {getWhiteOrBlackSq(i) ? 'white' : 'black'} {availableMoves.has(i) ? 'available' : ''}" data-i={i}>
				{#if sq}
					<div class="piece">
						<PieceElement piece={sq} />
					</div>
				{/if}
			</div>
		{/each}
	{/if}
</div>

<style>
	.board {
		display: grid;
		grid-template-columns: repeat(8, 1fr);
	}

	.piece {
		cursor: pointer;
	}
	.square {
		height: 50px;
		width: 50px;
		border: 1px solid black;
	}

	.square.black {
		background-color: rgb(87, 73, 148);
	}
	.square.white {
		background-color: rgb(140, 127, 190);
	}
	.square.available {
		filter: blur(2px);
	}
</style>
