<script>
    import Cell from "./Cell.svelte"

    const createGrid = (w, h) => {
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

<div>
    {#each grid as row}
        <div>
            {#each row as cell}
                <Cell />
            {/each}
        </div>
    {/each}
</div>

<style>

</style>