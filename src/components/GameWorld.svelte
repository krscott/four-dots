<script lang="ts">

import { fade } from "svelte/transition"
import { invoke } from "tauri/api/tauri"

import GameBoard from "./GameBoard.svelte"
import PlayerPieces from "./PlayerPieces.svelte"
import PlayerScore from "./PlayerScore.svelte"
import { Player1, Player2 } from "../gameBoardState"
import type { GameBoardState } from "../apiTypes"

export let gameBoardState: GameBoardState

invoke({
    cmd: "nop",
})

const resetBoard = () => {
    invoke({
        cmd: "clearBoard",
    })
}

const returnToTitle = () => {
    invoke({
        cmd: "returnToTitle"
    })
}


</script>

<div class="grid" transition:fade>
    <div class="gameboard-container">
        <GameBoard gameBoardState={gameBoardState} />
    </div>
    <div class="player1-pieces">
        <PlayerPieces player={Player1} gameBoardState={gameBoardState}/>
    </div>
    <div class="player2-pieces">
        <PlayerPieces player={Player2} gameBoardState={gameBoardState}/>
    </div>
    <div class="back-button-area">
        <button class="secondary-button emoji-font" on:click={returnToTitle}>◀</button>
    </div>
    <div class="footer">
        <div class="player1-score">
            <PlayerScore player={Player1} gameBoardState={gameBoardState} />
        </div>
        <button
            class="eject secondary-button emoji-font"
            on:click={resetBoard}
        >
            <span>⤵</span>
        </button>
        <div class="player2-score">
            <PlayerScore player={Player2} gameBoardState={gameBoardState} />
        </div>
    </div>
</div>

<style>

    .grid {
        text-align: center;
        padding: 1em;

        display: grid;
        grid-template-columns: 1fr auto 1fr;
        grid-template-rows: auto 1fr;
        grid-template-areas:
            "player1     gameboard  player2"
            "backbutton  footer     .";

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

    .back-button-area {
        grid-area: backbutton;
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