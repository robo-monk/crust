<script lang="ts">
	import { onMount } from 'svelte';
	import type { Piece } from '../chess/dtos/piece';
	import PieceElement from './Piece.svelte';
  import { Droppable } from '@shopify/draggable';

	export let squares: [Piece];

	const getWhiteOrBlackSq = (i: number) => {
		const getRank = (i) => Math.floor(i / 8);

		return (i + getRank(i)) % 2 == 0;
	};

	onMount(() => {
		const draggable = new Droppable(document.querySelectorAll('.board'), {
			draggable: '.piece',
      dropzone: '.square'
		});

    const getIndex = (e) => e?.data?.dropzone?.dataset.i

    const MOVE = {
      from: null,
      target: null,
      piece: null,
      captures: null
    };

    let move = MOVE;


    draggable.on('droppable:start', (e) => {
      move = MOVE;

      const from = getIndex(e);
      const piece = squares[from];

      move.from = from
      move.piece = piece
    });

    draggable.on('droppable:stop', (e) => {
      const target = getIndex(e);
      const capturedPiece = squares[target];

      move.target = target

      if (capturedPiece) {
        move.captures = capturedPiece;
      }

      console.log('move is', move)
    });
	});
</script>

<div class="board">
	{#each squares as sq, i}
		<div class="square {getWhiteOrBlackSq(i) ? 'white' : 'black'}" data-i={i}>
      {#if sq}
        <div class='piece'>
          <PieceElement piece={sq} />
        </div>
      {/if}
		</div>
	{/each}
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
</style>
