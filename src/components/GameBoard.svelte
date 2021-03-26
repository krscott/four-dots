<script lang="ts">

import { invoke } from "tauri/api/tauri"
import { fly } from "svelte/transition"
import { quadIn } from "svelte/easing"

import { Gbs } from "../gameBoardState"
import type { GameBoardState } from "../apiTypes"
import { CellEmptyVar, PlayerPlayer1Var, CellPlayer1PieceVar } from "../apiTypes"

export let gameBoardState: GameBoardState
export let singlePlayer: boolean

const isClickable = (r: number, c: number): boolean => {
    return (
        (!singlePlayer || gameBoardState.current_player.var === PlayerPlayer1Var) &&
        gameBoardState.winning_segment === null &&
        !Gbs.isSet(gameBoardState, r, c)
    )
}

const cellClickHandler = (r: number, c: number) => {
    if (isClickable(r, c)) {
        invoke({
            cmd: "putPieceInColumn",
            column: c,
        })
    }
}

const isInWinningSegment = (r: number, c: number): boolean => {
    if (gameBoardState.winning_segment === null) {
        return false
    }

    for (let point of gameBoardState.winning_segment.points) {
        if (c == point.x && r == point.y) {
            return true
        }
    }

    return false
}

</script>

<div class="gameboard">
    <div class="gameboard-inner bg-color">
        {#each [...Gbs.eachRowIndex(gameBoardState)] as r}
            <div class="row">
                {#each [...Gbs.eachCellInRow(gameBoardState, r)] as cell, c}
                    <div
                        class="cell"
                        class:cursor-pointer={isClickable(r, c)}
                        on:click={() => cellClickHandler(r, c)}
                    >
                        {#if cell.var !== CellEmptyVar}
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
                                    delay: 100 + (gameBoardState.height - r - 1)*20,
                                    easing: quadIn
                                }}"
                            >
                                <!-- Separate svg tags required to force Svelte redraw -->
                                {#if cell.var === CellPlayer1PieceVar}
                                    <svg
                                        class:glow={isInWinningSegment(r, c)}
                                        fill="var(--player1-color)"
                                    >
                                        <circle cx="50%" cy="50%" r="40%" />
                                    </svg>
                                {:else}
                                    <svg
                                        class:glow={isInWinningSegment(r, c)}
                                        fill="var(--player2-color)"
                                    >
                                        <circle cx="50%" cy="50%" r="40%" />
                                    </svg>
                                {/if}
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