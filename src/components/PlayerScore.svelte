<script lang="ts">

import { fade } from "svelte/transition"

import { Player, gameBoardState } from "../gameBoardState"
import Star from "./Star.svelte"

export let player: Player = Player.Player1

// eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
$: playerScore = (player == Player.Player1
    ? $gameBoardState.player1Score
    : $gameBoardState.player2Score
)

$: isWinningPlayer = (
    !!$gameBoardState.winningSegment &&
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
    $gameBoardState.winningSegment[0] == player
)

</script>

<div class="container" class:reversed={player == Player.Player2}>
    {#each Array(playerScore) as _, i}
        <span class="emoji-font" in:fade>
            <Star
                fill="var(--player{player}-color)"
				glow={isWinningPlayer && i == playerScore - 1}
            />
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