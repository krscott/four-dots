<script lang="ts">

import { fade } from "svelte/transition"
import { invoke } from "tauri/api/tauri"

import FitToScreen from "./components/FitToScreen.svelte"
import { appState } from "./appState"
import GameWorld from "./components/GameWorld.svelte"
import type { Difficulty } from "./apiTypes"
import {
    AppStateGameVsBotVar, AppStateGameVsPlayerVar, AppStateSelectDifficultyVar, AppStateTitleVar,
    DifficultyEasyVar, DifficultyMediumVar, DifficultyHardVar, DifficultyExpertVar
} from "./apiTypes"

const windowBaseWidth = 800
const windowBaseHeight = 600

invoke({
    cmd: "nop",
})

const start1P = (difficulty: Difficulty | null) => {
    invoke({
        cmd: "start1P",
        difficulty,
    })
}

const start2P = () => {
    invoke({
        cmd: "start2P"
    })
}

const returnToTitle = () => {
    invoke({
        cmd: "returnToTitle"
    })
}

</script>


<FitToScreen viewWidth={windowBaseWidth} viewHeight={windowBaseHeight}>
    {#if $appState.var === AppStateTitleVar}
        <div class="title-menu" transition:fade>
            <!-- <h1>Four Dots</h1> -->
            <div id="title-dots">
                {#each [1, 2, 2, 1] as x}
                    <svg fill="var(--player{x}-color)">
                        <circle cx="50%" cy="50%" r="40%" />
                    </svg>
                {/each}
            </div>
            <button class="secondary-button emoji-font" on:click={() => start1P(null)}>😛 / 🤖</button>
            <button class="secondary-button emoji-font" on:click={start2P}>😛 / 😜</button>
            <!-- <button class="secondary-button emoji-font" on:click={start2P}>😛 / 🌐</button> -->
        </div>
    {/if}
    {#if $appState.var === AppStateSelectDifficultyVar}
        <div class="difficulty-menu" transition:fade>
            <h1 class="emoji-font">🤖</h1>
            <div>
                <button
                    class="secondary-button emoji-font"
                    on:click={() => start1P({ var: DifficultyEasyVar })}
                >
                    👶
                </button>
                <button
                    class="secondary-button emoji-font"
                    on:click={() => start1P({ var: DifficultyMediumVar })}
                >
                    🙂
                </button>
                <button
                    class="secondary-button emoji-font"
                    on:click={() => start1P({ var: DifficultyHardVar })}
                >
                    😓
                </button>
                <button
                    class="secondary-button emoji-font"
                    on:click={() => start1P({ var: DifficultyExpertVar })}
                >
                    😈
                </button>
            </div>

            <div>
                <button
                    class="secondary-button emoji-font"
                    on:click={returnToTitle}
                >
                    ◀
                </button>
            </div>
        </div>
    {/if}
    {#if $appState.var === AppStateGameVsBotVar || $appState.var === AppStateGameVsPlayerVar}
        <GameWorld
            gameBoardState={$appState.vardata}
            singlePlayer={$appState.var === AppStateGameVsBotVar}
        />
    {/if}
</FitToScreen>

<style>

    .title-menu, .difficulty-menu {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        margin: 0;
        text-align: center;
        color: var(--text-em-color);
    }

    #title-dots svg {
        display: inline-block;
        width: 9rem;
        height: 9rem;
    }

    /* h1 {
        margin: 0;
    } */

    button {
        font-size: xx-large;
        margin: auto;
        margin-top: 1rem;
        width: 9rem;
    }

    .title-menu button {
        display: block;
        width: 9rem;
    }

    .difficulty-menu button {
        display: inline-block;
        width: 4rem;
    }

    .difficulty-menu h1 {
        font-size: 4rem;
        margin: 0;
    }

</style>