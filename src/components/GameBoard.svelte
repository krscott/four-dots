<script lang="ts">
	import { invoke } from "tauri/api/tauri"
    import { fly } from "svelte/transition"
    import { quadIn } from "svelte/easing"

    import { state, MaybePlayer } from "../state"

    const cell_click_handler = (r: number, c: number) => {
        if (!$state.is_set(r, c)) {
            invoke({
                cmd: "putPieceInColumn",
                column: c,
            })
        }
    }
</script>

<div class="gameboard">
    <div class="gameboard-inner bg-color">
        {#each [...$state.each_row_index()] as r}
            <div class="row">
                {#each [...$state.each_cell_in_row(r)] as cell, c}
                    <div
                        class="cell"
                        class:cursor-pointer={!$state.is_set(r, c)}
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
                                    delay: 100 + ($state.height - r - 1)*20,
                                    easing: quadIn
                                }}"
                            >
                                <svg fill="var(--player{cell}-color)">
                                    <circle cx="50%" cy="50%" r="40%" />
                                </svg>
                            </div>
                        {/if}
                        <div class="svg-container">
                            <svg class="mask">
                                <mask id="cutout">
                                    <rect width="100%" height="100%" fill="white"></rect>
                                    <circle cx="50%" cy="50%" r="40%" fill="black"/>
                                </mask>
                                <rect width="100%" height="100%" mask="url(#cutout)"></rect>
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
</style>