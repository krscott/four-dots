<script lang="ts">
	import GameBoard from "./components/GameBoard.svelte"

    import { state_store, State } from "./state"

	const reset_board = () => {
		state_store.update(state => {
			state.clear()
			return state
		})
	}
</script>

<main>
	<h1 class="text-em-color">Four Dots!</h1>
	<GameBoard />
	<div>
		<button class="secondary-button emoji-font" on:click={reset_board}>
			<span>⤵</span>
		</button>
	</div>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
	}

	button {
		position: relative;
		width: 3rem;
		height: 3rem;
	}

	button span {
		font-size: xx-large;
		position: absolute;
		transform: translate(-50%, calc(-50% - 0.3rem));
	}
</style>