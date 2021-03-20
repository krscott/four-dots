import { writable } from "svelte/store"

export enum Cell {
    None,
    Player1,
    Player2,
}

export class State {
    #cells: Cell[]
    #width: number
    #height: number

    currentPlayer: Cell

    constructor(width: number, height: number) {
        const cells = new Array(height * width).fill(Cell.None)

        this.#cells = cells
        this.#width = width
        this.#height = height
        this.currentPlayer = Cell.Player1;
    }

    clear() {
        this.#cells.fill(Cell.None)
    }

    width(): number {
        return this.#width
    }

    height(): number {
        return this.#height
    }

    isValidCoord(row: number, column: number): boolean {
        return (
            row >= 0 &&
            row < this.#height &&
            column >= 0 &&
            column < this.#width
        )
    }

    coordToIndex(row: number, column: number): number {
        return row * this.#width + column
    }

    get(row: number, column: number): Cell {
        if (this.isValidCoord(row, column)) {
            return this.#cells[this.coordToIndex(row, column)]
        } else {
            return Cell.None
        }
    }

    set(row: number, column: number, value: Cell) {
        if (this.isValidCoord(row, column)) {
            this.#cells[this.coordToIndex(row, column)] = value
        }
    }

    is_set(row: number, column: number): boolean {
        return this.get(row, column) !== Cell.None
    }

    * each_row_index() {
        for (let r = 0; r < this.#height; ++r) {
            yield r
        }
    }

    * each_cell_in_row(row: number) {
        for (let c = 0; c < this.#width; ++c) {
            yield this.#cells[this.coordToIndex(row, c)]
        }
    }

    put_piece_in_column(column: number): boolean {
        if (column < 0 || column >= this.width()) {
            return false
        }

        for (let r = this.height() - 1; r >= 0; --r) {
            if (!this.is_set(r, column)) {
                this.set(r, column, this.currentPlayer)
                this.advanceTurn()
                return true
            }
        }

        return false
    }

    advanceTurn() {
        switch (this.currentPlayer) {
            case Cell.Player1:
                this.currentPlayer = Cell.Player2
                break
            default:
                this.currentPlayer = Cell.Player1
        }
    }

    getRemainingPiecesCount(player: Cell): number {
        let emptyCellsCount = this.#cells.filter(cell => cell === Cell.None).length

        if (player === this.currentPlayer) {
            return Math.ceil(emptyCellsCount / 2)
        } else {
            return Math.floor(emptyCellsCount / 2)
        }
    }
}

export const state = writable(new State(7, 6))
