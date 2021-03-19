<script lang="ts">
    import { fly } from "svelte/transition"
    import { quadIn } from 'svelte/easing'


    import { Grid } from "../grid"

    let grid: Grid = new Grid(7, 6)
    let currentPlayer = 1;
</script>

<div class="gameboard">
    <div class="gameboard-inner bg-color">
        {#each grid.rows as row, r}
            <div class="row">
                {#each row as cell, c}
                    <div class="cell" on:click={() => {
                        if (grid.get(r, c) === 0) {
                            grid.set(r, c, currentPlayer)
                            currentPlayer = currentPlayer % 2 + 1
                        } else {
                            grid.set(r, c, 0)
                        }
                        grid = grid
                    }}>
                        {#if cell !== 0}
                            <div
                                class="piece svg-container"
                                in:fly="{{y: -400, duration: 300, easing: quadIn}}"
                                out:fly="{{y: 400, duration: 300, easing: quadIn}}"
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
        cursor: pointer;
        width: 4rem;
        height: 4rem;
    }

    .svg-container {
        vertical-align: top;
        display: inline-block;
        position: relative;
    }

    svg {
        position: relative;
        width: 4rem;
        height: 4rem;
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