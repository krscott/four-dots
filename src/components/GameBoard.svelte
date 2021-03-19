<script lang="ts">

    const createGrid = (w: number, h: number): number[][] => {
        const grid = new Array(h)

        for (let i = 0; i < h; ++i) {
            grid[i] = new Array(w)

            for (let j = 0; j < w; ++j) {
                grid[i][j] = 0
            }
        }

        return grid
    }

    const grid = createGrid(7, 6)
</script>

<div class="gameboard panel-bg-color">
    <div class="gameboard-inner bg-color">
        {#each grid as row}
            <div class="row">
                {#each row as cell}
                    <div class="cell">
                        <svg class="panel-fill-color">
                            <mask id="cutout">
                                <rect width="100%" height="100%" fill="white"></rect>
                                <circle cx="50%" cy="50%" r="40%" fill="black"/>
                            </mask>
                            <rect width="100%" height="100%" mask="url(#cutout)"></rect>
                        </svg>
                    </div>
                {/each}
            </div>
        {/each}
    </div>
</div>

<style>
    .gameboard {
        padding: 1rem;
        border-radius: 3rem;
        display: inline-block;
    }

    .gameboard-inner {
        display: inline-block;
    }

    .row {
        white-space: nowrap;
    }

    .cell {
        /* height: 32px; */
        display: inline-block;
    }

    .cell svg {
        width: 4rem;
        height: 4rem;
        vertical-align: top;
    }
</style>