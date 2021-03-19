
export class Grid {
    rows: number[][]

    constructor(w: number, h: number) {
        const rows = new Array(h)
        for (let i = 0; i < h; ++i) {
            rows[i] = new Array(w)
        }

        this.rows = rows
        this.clear()
    }

    width(): number {
        if (this.rows.length == 0) {
            return 0;
        } else {
            return this.rows[0].length
        }
    }

    height(): number {
        return this.rows.length
    }

    get(row: number, column: number): number {
        return this.rows[row][column]
    }

    set(row: number, column: number, value: number) {
        this.rows[row][column] = value
    }

    is_set(row: number, column: number): boolean {
        return this.rows[row][column] !== 0
    }

    drop_in_column(column: number, value: number): boolean {
        if (this.width() <= column) {
            return false
        }

        for (let r = this.height() - 1; r >= 0; --r) {
            if (!this.is_set(r, column)) {
                this.set(r, column, value)
                return true
            }
        }

        return false
    }

    clear() {
        let w = this.width()
        let h = this.height()

        for (let i = 0; i < h; ++i) {
            for (let j = 0; j < w; ++j) {
                this.rows[i][j] = 0
            }
        }
    }
}

