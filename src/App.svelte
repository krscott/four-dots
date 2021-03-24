<script lang="ts">

import { invoke } from "tauri/api/tauri"

import GameBoard from "./components/GameBoard.svelte"
import PlayerPieces from "./components/PlayerPieces.svelte"
import PlayerScore from "./components/PlayerScore.svelte"
import FitToScreen from "./components/FitToScreen.svelte"
import { appState } from "./appState"
import { Player1, Player2 } from "./gameBoardState"

const windowBaseWidth = 800
const windowBaseHeight = 600

invoke({
    cmd: "nop",
})

const resetBoard = () => {
    invoke({
        cmd: "clearBoard",
    })
}

const startGame = () => {
    invoke({
        cmd: "startGame"
    })
}

</script>


<FitToScreen viewWidth={windowBaseWidth} viewHeight={windowBaseHeight}>
    {#if $appState.var === "Title"}
        <div class="title">
            <h1>Four Dots</h1>
            <button on:click={startGame}>🧍‍♂️🧍‍♂️</button>
        </div>
    {/if}
    {#if $appState.var === "Game"}
        <div class="grid">
            <div class="gameboard-container">
                <GameBoard gameBoardState={$appState.vardata} />
            </div>
            <div class="player1-pieces">
                <PlayerPieces player={Player1} gameBoardState={$appState.vardata}/>
            </div>
            <div class="player2-pieces">
                <PlayerPieces player={Player2} gameBoardState={$appState.vardata}/>
            </div>
            <div class="footer">
                <div class="player1-score">
                    <PlayerScore player={Player1} gameBoardState={$appState.vardata} />
                </div>
                <button
                    class="eject secondary-button emoji-font"
                    on:click={resetBoard}
                >
                    <span>⤵</span>
                </button>
                <div class="player2-score">
                    <PlayerScore player={Player2} gameBoardState={$appState.vardata} />
                </div>
            </div>
        </div>
    {/if}
</FitToScreen>

<style>

    .title {
        text-align: center;
    }

    .grid {
        text-align: center;
        padding: 1em;

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