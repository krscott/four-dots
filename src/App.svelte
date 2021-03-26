<script lang="ts">

import { invoke } from "tauri/api/tauri"

import FitToScreen from "./components/FitToScreen.svelte"
import { appState } from "./appState"
import GameWorld from "./components/GameWorld.svelte"
import { AppStateGameVsBotVar, AppStateGameVsPlayerVar, AppStateTitleVar } from "./apiTypes"

const windowBaseWidth = 800
const windowBaseHeight = 600

invoke({
    cmd: "nop",
})

const start1P = () => {
    invoke({
        cmd: "start1P"
    })
}

const start2P = () => {
    invoke({
        cmd: "start2P"
    })
}

</script>


<FitToScreen viewWidth={windowBaseWidth} viewHeight={windowBaseHeight}>
    {#if $appState.var === AppStateTitleVar}
        <div class="title">
            <h1>Four Dots</h1>
            <button class="secondary-button emoji-font" on:click={start1P}>😛 / 🤖</button>
            <button class="secondary-button emoji-font" on:click={start2P}>😛 / 😜</button>
            <!-- <button class="secondary-button emoji-font" on:click={start2P}>😛 / 🌐</button> -->
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

    .title {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        margin: 0;
        text-align: center;
        color: var(--text-em-color);
    }

    h1 {
        margin: 0;
    }

    button {
        display: block;
        font-size: xx-large;
        margin: auto;
        margin-top: 1rem;
        width: 9rem;
    }

</style>