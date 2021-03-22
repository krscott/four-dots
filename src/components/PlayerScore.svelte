<script lang="ts">
    import { fade } from "svelte/transition"

    import { Player, state } from "../state"
    import Star from "./Star.svelte"

    export let player: Player = Player.Player1

    $: playerScore = (player == Player.Player1
        ? $state.player1Score
        : $state.player2Score
    )
</script>

<div class="container" class:reversed={player == Player.Player2}>
    {#each Array(playerScore) as _}
        <span class="emoji-font" transition:fade>
            <Star fill="var(--player{player}-color)"/>
        </span>
    {/each}
</div>

<style>
    .container {
        display: flex;
        flex-wrap: wrap;
    }

    .reversed {
        flex-direction: row-reverse;
    }
</style>