<script lang="ts">
    import { fly } from "svelte/transition"
    import { quadIn } from "svelte/easing"

    import { state_store, State, Cell } from "../state"
    import { onDestroy } from "svelte"

    export let player = Cell.Player1

    let state: State

    // const unsubscribe_game_state = state_store.subscribe(s => {
    //     state = s
    // })
    // onDestroy(unsubscribe_game_state)


</script>

<div class="pieces-stack" style="--player-color: var(--player{player}-color)">
    <div class="spacer"></div>
    {#each Array($state_store.getRemainingPiecesCount(player)) as _, i}
        <div
            class="piece"
            out:fly="{{y: -600, duration: 200, easing: quadIn}}"
            in:fly="{{y: -600, duration: 200, delay: 300 + i*20, easing: quadIn}}"
        ></div>
    {/each}
</div>

<style>
    .pieces-stack {
        display: flex;
        flex-direction: column-reverse;
        width: 100%;
        height: 100%;
    }

    .piece {
        margin-left: auto;
        margin-right: auto;
        width: 4.7rem;
        min-height: 1rem;
        background-color: var(--player-color);
        margin-top: 0.2rem;
        border-radius: 0.2rem;
    }
/*
    .piece:last-child {
    } */
</style>