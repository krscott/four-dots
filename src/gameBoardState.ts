import { writable } from "svelte/store"

export enum MaybePlayer {
    None = 0,
    Player1 = 1,
    Player2 = 2,
}

export enum Player {
    Player1 = 1,
    Player2 = 2,
}

export class GameBoardState {
    cells: MaybePlayer[]
    width: number
    height: number
    currentPlayer: Player
    winningSegment: [Player, [number, number][]] | null
    player1Score: number
    player2Score: number

    constructor(width: number, height: number) {
        const cells = new Array<MaybePlayer>(height * width).fill(MaybePlayer.None)

        this.cells = cells
        this.width = width
        this.height = height
        this.currentPlayer = Player.Player1
        this.winningSegment = null
        this.player1Score = 0
        this.player2Score = 0
    }

    clear(): void {
        this.cells.fill(MaybePlayer.None)
    }

    isValidCoord(row: number, column: number): boolean {
        return (
            row >= 0 &&
            row < this.height &&
            column >= 0 &&
            column < this.width
        )
    }

    coordToIndex(row: number, column: number): number {
        return row * this.width + column
    }

    get(row: number, column: number): MaybePlayer {
        if (this.isValidCoord(row, column)) {
            return this.cells[this.coordToIndex(row, column)]
        } else {
            return MaybePlayer.None
        }
    }

    isSet(row: number, column: number): boolean {
        return this.get(row, column) !== MaybePlayer.None
    }

    * eachRowIndex(): IterableIterator<number> {
        for (let r = 0; r < this.height; ++r) {
            yield r
        }
    }

    * eachCellInRow(row: number): IterableIterator<number> {
        for (let c = 0; c < this.width; ++c) {
            yield this.cells[this.coordToIndex(row, c)]
        }
    }

    getRemainingPiecesCount(player: Player): number {
        const emptyCellsCount = this.cells.filter(cell => cell === MaybePlayer.None).length

        if (player === this.currentPlayer) {
            return Math.ceil(emptyCellsCount / 2)
        } else {
            return Math.floor(emptyCellsCount / 2)
        }
    }
}

export const gameBoardState = writable(new GameBoardState(7, 6))
