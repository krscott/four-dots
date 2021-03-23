<script lang="ts">

import { fade } from "svelte/transition"

import { GameBoardState, Player } from "../gameBoardState"
import Star from "./Star.svelte"

export let player: Player = Player.Player1
export let gameBoardState: GameBoardState

$: playerScore = (player == Player.Player1
    ? gameBoardState.player1Score
    : gameBoardState.player2Score
)

$: isWinningPlayer = gameBoardState.winningSegment.map(ws => ws[0] == player).unwrapOr(false)

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