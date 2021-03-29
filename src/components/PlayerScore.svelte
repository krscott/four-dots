<script lang="ts">

import { fade } from "svelte/transition"

import type { GameBoardState, Player } from "../apiTypes"
import { matchPlayer } from "../apiTypes"
import Star from "./Star.svelte"
import { playerInt } from "../gameBoardState"

export let player: Player
export let gameBoardState: GameBoardState

$: playerScore = matchPlayer(player, {
    Player1: () => gameBoardState.player1_score,
    Player2: () => gameBoardState.player2_score
})

$: reversed = matchPlayer(player, {
    Player1: () => false,
    Player2: () => true,
})

$: isWinningPlayer = (gameBoardState.winning_segment == null
    ? false
    : gameBoardState.winning_segment.player.var === player.var
)

</script>

<div class="container" class:reversed={reversed}>
    {#each Array(playerScore) as _, i}
        <span class="emoji-font" in:fade>
            <Star
                fill="var(--player{playerInt(player)}-color)"
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