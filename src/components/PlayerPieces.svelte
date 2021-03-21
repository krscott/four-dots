<script lang="ts">
    import { fly } from "svelte/transition"
    import { quadIn } from "svelte/easing"

    import { state, MaybePlayer } from "../state"

    export let player = MaybePlayer.Player1

</script>

<div
    class="pieces-stack"
    class:is-current-player-stack={$state.currentPlayer == player}
    style="--player-color: var(--player{player}-color)"
>
    <div class="spacer"></div>
    {#each Array($state.getRemainingPiecesCount(player)) as _, i}
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

    .is-current-player-stack .piece:last-child {
        animation:
            hover-animation 10s infinite ease-in-out;
    }

    @keyframes hover-animation {
        0% {transform: translateY(0);}
        30% {transform: translateY(-1rem);}
        50% {transform: translateY(-0.3rem);}
        70% {transform: translateY(-1rem);}
        100% {transform: translateY(0);}
    }
</style>