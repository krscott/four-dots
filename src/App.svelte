<script lang="ts">
	import { listen, emit } from "tauri/api/event"
	import { invoke } from "tauri/api/tauri"

	import GameBoard from "./components/GameBoard.svelte"
	import PlayerPieces from "./components/PlayerPieces.svelte"
    import { state } from "./state"

	const reset_board = () => {
		state.update(state => {
			state.clear()
			return state
		})
	}

	invoke({
		cmd: "myCustomCommand",
		argument: "Hello from Svelte!",
	})
</script>

<main>
	<div class="grid">
		<!-- <h1 class="text-em-color">Four Dots!</h1> -->
		<div class="gameboard-container">
			<GameBoard />
		</div>
		<div class="player1">
			<PlayerPieces player={1}/>
		</div>
		<div class="player2">
			<PlayerPieces player={2}/>
		</div>
		<div class="controls">
			<button class="secondary-button emoji-font" on:click={reset_board}>
				<span>⤵</span>
			</button>
		</div>
	</div>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		/* margin: 0 auto; */
	}

	.grid {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		grid-template-rows: auto 1fr;
		grid-template-areas:
			"player1 gameboard player2"
			"controls controls controls";

 		gap: 1em;
	}

	.gameboard-container {
		grid-area: gameboard;
	}

	.player1 {
		grid-area: player1;
	}

	.player2 {
		grid-area: player2;
	}

	.controls {
		grid-area: controls;
	}

	button {
		position: relative;
		width: 3rem;
		height: 3rem;
	}

	button.emoji-font span {
		font-size: xx-large;
		position: absolute;
		transform: translate(-50%, calc(-50% - 0.3rem));
	}
</style>