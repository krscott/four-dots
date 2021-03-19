import { writable } from "svelte/store"

export class State {
    rows: number[][]
    currentPlayer: number

    constructor(w: number, h: number) {
        const rows = new Array(h)
        for (let i = 0; i < h; ++i) {
            rows[i] = new Array(w)
        }

        this.rows = rows
        this.clear()

        this.currentPlayer = 1;
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

    is_valid_coord(row: number, column: number): boolean {
        return (
            row >= 0 &&
            row < this.rows.length &&
            column >= 0 &&
            column < this.rows[row].length
        )
    }

    get(row: number, column: number): number {
        if (this.is_valid_coord(row, column)) {
            return this.rows[row][column]
        } else {
            return 0
        }
    }

    set(row: number, column: number, value: number) {
        if (this.is_valid_coord(row, column)) {
            this.rows[row][column] = value
        }
    }

    is_set(row: number, column: number): boolean {
        return this.get(row, column) !== 0
    }

    put_piece_in_column(column: number): boolean {
        if (column < 0 || column >= this.width()) {
            return false
        }

        for (let r = this.height() - 1; r >= 0; --r) {
            if (!this.is_set(r, column)) {
                this.set(r, column, this.currentPlayer)
                this.skip_turn()
                return true
            }
        }

        return false
    }

    skip_turn() {
        this.currentPlayer = this.currentPlayer % 2 + 1
    }
}

export const state_store = writable(new State(7, 6))
