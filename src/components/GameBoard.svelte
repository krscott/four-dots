<script lang="ts">
/* eslint-disable
    @typescript-eslint/no-unsafe-member-access,
    @typescript-eslint/no-unsafe-call,
    @typescript-eslint/no-unsafe-assignment */

import { invoke } from "tauri/api/tauri"
import { fly } from "svelte/transition"
import { quadIn } from "svelte/easing"

import { gameBoardState, MaybePlayer } from "../gameBoardState"

$: isGameOver = !!$gameBoardState.winningSegment

const cell_click_handler = (r: number, c: number) => {
    if (!isGameOver && !$gameBoardState.isSet(r, c)) {
        invoke({
            cmd: "putPieceInColumn",
            column: c,
        })
    }
}

const is_in_winning_segment = (r: number, c: number): boolean => {
    if (!$gameBoardState.winningSegment) {
        return false
    }

    for (let [wr, wc] of $gameBoardState.winningSegment[1]) {
        if (r == wr && c == wc) {
            return true
        }
    }

    return false
}

</script>

<div class="gameboard">
    <div class="gameboard-inner bg-color">
        {#each [...$gameBoardState.eachRowIndex()] as r}
            <div class="row">
                {#each [...$gameBoardState.eachCellInRow(r)] as cell, c}
                    <div
                        class="cell"
                        class:cursor-pointer={!isGameOver && !$gameBoardState.isSet(r, c)}
                        on:click={() => cell_click_handler(r, c)}
                    >
                        {#if cell !== MaybePlayer.None}
                            <div
                                class="piece svg-container"
                                in:fly="{{
                                    y: -600,
                                    duration: 300,
                                    delay: 100,
                                    easing: quadIn
                                }}"
                                out:fly="{{
                                    y: 600,
                                    duration: 300,
                                    delay: 100 + ($gameBoardState.height - r - 1)*20,
                                    easing: quadIn
                                }}"
                            >
                                <svg
                                    class:glow={is_in_winning_segment(r, c)}
                                    fill="var(--player{cell}-color)"
                                >
                                    <circle cx="50%" cy="50%" r="40%" />
                                </svg>
                            </div>
                        {/if}
                        <div class="svg-container">
                            <svg class="mask">
                                <mask id="cutout-{r}-{c}">
                                    <rect width="100%" height="100%" fill="white"></rect>
                                    <circle cx="50%" cy="50%" r="40%" fill="black"/>
                                </mask>
                                <rect width="100%" height="100%" mask="url(#cutout-{r}-{c})"></rect>
                            </svg>
                        </div>
                    </div>
                {/each}
            </div>
        {/each}
    </div>

    <div class="gameboard-border">

    </div>
</div>

<style>
    .gameboard {
        --gameboard-border-width: 1rem;
        --gameboard-cell-size: 4.7rem;

        display: inline-block;
        position: relative;
        margin: var(--gameboard-border-width);
    }

    .gameboard-border {
        position: absolute;
        top: calc(-1 * var(--gameboard-border-width));
        left: calc(-1 * var(--gameboard-border-width));
        width: 100%;
        height: 100%;
        border-color: var(--panel-bg-color);
        border-radius: 3rem;
        border-width: var(--gameboard-border-width);
        border-style: solid;
    }

    .gameboard-inner {
        display: inline-block;
        background-color: var(--bg-color);
    }

    .row {
        white-space: nowrap;
    }

    .cell {
        display: inline-block;
        position: relative;
        width: var(--gameboard-cell-size);
        height: var(--gameboard-cell-size);
    }

    .svg-container {
        vertical-align: top;
        display: inline-block;
        position: relative;
    }

    svg {
        position: relative;
        width: var(--gameboard-cell-size);
        height: var(--gameboard-cell-size);
        vertical-align: top;
        z-index: 1;
    }

    .mask {
        fill: var(--panel-bg-color);
    }

    .piece {
        position: absolute;
    }

    /* Fix scale rounding issue */
    .gameboard-inner, .cell {
        margin: -1px;
    }

</style>