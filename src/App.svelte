<script lang="ts">

	import { invoke } from "tauri/api/tauri"

	import GameBoard from "./components/GameBoard.svelte"
	import PlayerPieces from "./components/PlayerPieces.svelte"
	import PlayerScore from "./components/PlayerScore.svelte";
    import { State, state } from "./state"

	invoke({
		cmd: "nop",
	})

	const reset_board = () => {
		invoke({
			cmd: "clearBoard",
		})
	}

	;(window as any).rust_error_handler = (err: any) => {
		console.error(err)
	}

	;(window as any).rust_set_state = (new_state: State) => {
		state.update(state => {
			Object.assign(state, new_state)
			return state
		})
	}
</script>

<main>
	<div class="grid">
		<!-- <h1 class="text-em-color">Four Dots!</h1> -->
		<div class="gameboard-container">
			<GameBoard />
		</div>
		<div class="player1-pieces">
			<PlayerPieces player={1}/>
		</div>
		<div class="player2-pieces">
			<PlayerPieces player={2}/>
		</div>
		<div class="footer">
			<div class="player1-score">
				<PlayerScore player={1} />
			</div>
			<button
				class="eject secondary-button emoji-font"
				on:click={reset_board}
			>
				<span>⤵</span>
			</button>
			<div class="player2-score">
				<PlayerScore player={2} />
			</div>
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
			". footer .";

 		gap: 1em;
	}

	.gameboard-container {
		grid-area: gameboard;
	}

	.player1-pieces {
		grid-area: player1;
	}

	.player2-pieces {
		grid-area: player2;
	}

	.footer {
		grid-area: footer;

		display: grid;
		grid-template-columns: 1fr auto 1fr;
		grid-template-areas:
			"player1 eject player2";
	}

	.player1-score {
		grid-area: player1;
		margin: auto;
		margin-left: 0;
	}

	.eject {
		grid-area: eject;
	}

	.player2-score {
		grid-area: player2;
		margin: auto;
		margin-right: 0;
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