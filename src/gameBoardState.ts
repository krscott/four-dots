
import { Ok, Result, Option, None, Err, Some } from "@hqoss/monads"
import { errStr, expectArray, expectNumber } from "./util"

export enum MaybePlayer {
    None = 0,
    Player1 = 1,
    Player2 = 2,
}

const expectMaybePlayer = (value: unknown): Result<MaybePlayer, Error> => {
    if (MaybePlayer[value as number] !== undefined) {
        return Ok(value as MaybePlayer)
    } else {
        return errStr("Expected MaybePlayer")
    }
}

export enum Player {
    Player1 = 1,
    Player2 = 2,
}

const expectPlayer = (value: unknown): Result<Player, Error> => {
    if (Player[value as number] !== undefined) {
        return Ok(value as Player)
    } else {
        return errStr("Expected Player")
    }
}

type PlayerSegment = [Player, Array<[number, number]>]

const expectNumberPair = (value: unknown): Result<[number, number], Error> => {
    if (!(value instanceof Array)) {
        return errStr("Expected Array")
    }
    if (value.length != 2) {
        return errStr("Expected array of length 2")
    }
    for (const x of value) {
        if (typeof x !== "number") {
            return errStr("Expected array of numbers")
        }
    }
    return Ok(value as [number, number])
}

const expectWinningSegment = (value: unknown): Result<Option<PlayerSegment>, Error> => {
    if (value === null || value === undefined) {
        return Ok(None)
    }
    if (!(value instanceof Array)) {
        return errStr("Expected Array")
    }
    if (value.length !== 2) {
        return errStr("Expected array of length 2")
    }
    const resultPlayer = expectPlayer(value[0])
    if (resultPlayer.isErr()) {
        return Err(resultPlayer.unwrapErr())
    }
    const resultSegment = expectArray(value[1], expectNumberPair)
    if (resultSegment.isErr()) {
        return Err(resultSegment.unwrapErr())
    }

    return Ok(Some(value as PlayerSegment))
}

export class GameBoardState {
    cells: Array<MaybePlayer>
    width: number
    height: number
    currentPlayer: Player
    winningSegment: Option<PlayerSegment>
    player1Score: number
    player2Score: number

    constructor(width: number, height: number) {
        const cells = new Array<MaybePlayer>(height * width).fill(MaybePlayer.None)

        this.cells = cells
        this.width = width
        this.height = height
        this.currentPlayer = Player.Player1
        this.winningSegment = None
        this.player1Score = 0
        this.player2Score = 0
    }

    updateFromRecord(data: Record<string, unknown>): Result<void, Error> {

        const cellsRes = expectArray(data.cells, expectMaybePlayer)
        if (cellsRes.isErr()) { return Err(cellsRes.unwrapErr()) }

        const widthRes = expectNumber(data.width)
        if (widthRes.isErr()) { return Err(widthRes.unwrapErr()) }

        const heightRes = expectNumber(data.height)
        if (heightRes.isErr()) { return Err(heightRes.unwrapErr()) }

        const currentPlayerRes = expectPlayer(data.currentPlayer)
        if (currentPlayerRes.isErr()) { return Err(currentPlayerRes.unwrapErr()) }

        const winningSegmentRes = expectWinningSegment(data.winningSegment)
        if (winningSegmentRes.isErr()) { return Err(winningSegmentRes.unwrapErr()) }

        const player1ScoreRes = expectNumber(data.player1Score)
        if (player1ScoreRes.isErr()) { return Err(player1ScoreRes.unwrapErr()) }

        const player2ScoreRes = expectNumber(data.player2Score)
        if (player2ScoreRes.isErr()) { return Err(player2ScoreRes.unwrapErr()) }

        this.cells = cellsRes.unwrap()
        this.width = widthRes.unwrap()
        this.height = heightRes.unwrap()
        this.currentPlayer = currentPlayerRes.unwrap()
        this.winningSegment = winningSegmentRes.unwrap()
        this.player1Score = player1ScoreRes.unwrap()
        this.player2Score = player2ScoreRes.unwrap()

        return Ok(undefined)
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
