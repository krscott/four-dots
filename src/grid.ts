
export class Grid {
    rows: number[][]

    constructor(w: number, h: number) {
        const rows = new Array(h)

        for (let i = 0; i < h; ++i) {
            rows[i] = new Array(w)

            for (let j = 0; j < w; ++j) {
                rows[i][j] = 0
            }
        }

        this.rows = rows
    }

    get(row: number, col: number): number {
        return this.rows[row][col]
    }

    set(row: number, col: number, value: number) {
        this.rows[row][col] = value
    }
}

