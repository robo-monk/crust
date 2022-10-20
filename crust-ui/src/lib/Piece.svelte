<script lang="ts">
	import type { Piece } from '../chess/dtos/piece';

	import WhitePawn from '../assets/pieces/white-pawn.svg';
	import WhiteBishop from '../assets/pieces/white-bishop.svg';
	import WhiteKnight from '../assets/pieces/white-knight.svg';
	import WhiteRook from '../assets/pieces/white-rook.svg';
	import WhiteQueen from '../assets/pieces/white-queen.svg';
	import WhiteKing from '../assets/pieces/white-king.svg';

	import BlackPawn from '../assets/pieces/black-pawn.svg';
	import BlackBishop from '../assets/pieces/black-bishop.svg';
	import BlackKnight from '../assets/pieces/black-knight.svg';
	import BlackRook from '../assets/pieces/black-rook.svg';
	import BlackQueen from '../assets/pieces/black-queen.svg';
	import BlackKing from '../assets/pieces/black-king.svg';

	const PIECES = {
		WhitePawn,
		WhiteBishop,
		WhiteKnight,
		WhiteRook,
		WhiteQueen,
		WhiteKing,

		BlackPawn,
		BlackBishop,
		BlackKnight,
		BlackRook,
		BlackQueen,
		BlackKing,
	};

	export let piece: Piece;

	let src = BlackBishop;

	$: {
		(async () => {
			if (piece) {
				console.log('piece', piece);
				src = PIECES[`${piece.color}${piece.class}`];
				console.log('src is', src);
			} else {
				src = null;
			}
		})();
	}
</script>

<div>
	{#if src}
		<img {src} alt={src} />
	{/if}
</div>
