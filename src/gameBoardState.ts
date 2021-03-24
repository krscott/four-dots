import {
    GameBoardState, Cell, CellEmptyVar, CellPlayer1PieceVar, CellPlayer2PieceVar,
    Player, PlayerPlayer1Var, PlayerPlayer2Var
} from "./apiTypes"


export const Player1: Player = { var: PlayerPlayer1Var }
export const Player2: Player = { var: PlayerPlayer2Var }
export const EmptyCell: Cell = { var: CellEmptyVar }

export const playerInt = (input: Cell | Player): number => {
    switch (input.var) {
        case CellPlayer1PieceVar: case PlayerPlayer1Var:
            return 1
        case CellPlayer2PieceVar: case PlayerPlayer2Var:
            return 2
        default:
            return 0
    }
}

export const Gbs = {
    create(width: number, height: number): GameBoardState {
        const cells = new Array<Cell>(height * width).fill(EmptyCell)

        return {
            tick: 0,
            cells,
            width,
            height,
            current_player: Player1,
            winning_segment: null,
            player1_score: 0,
            player2_score: 0,
        }
    },

    clear(gbs: GameBoardState): void {
        gbs.cells.fill(EmptyCell)
    },

    isValidCoord(gbs: GameBoardState, row: number, column: number): boolean {
        return (
            row >= 0 &&
            row < gbs.height &&
            column >= 0 &&
            column < gbs.width
        )
    },

    coordToIndex(gbs: GameBoardState, row: number, column: number): number {
        return row * gbs.width + column
    },

    get(gbs: GameBoardState, row: number, column: number): Cell {
        if (Gbs.isValidCoord(gbs, row, column)) {
            return gbs.cells[Gbs.coordToIndex(gbs, row, column)]
        } else {
            return EmptyCell
        }
    },

    isSet(gbs: GameBoardState, row: number, column: number): boolean {
        return Gbs.get(gbs, row, column).var !== CellEmptyVar
    },

    * eachRowIndex(gbs: GameBoardState): IterableIterator<number> {
        for (let r = 0; r < gbs.height; ++r) {
            yield r
        }
    },

    * eachCellInRow(gbs: GameBoardState, row: number): IterableIterator<Cell> {
        for (let c = 0; c < gbs.width; ++c) {
            yield gbs.cells[Gbs.coordToIndex(gbs, row, c)]
        }
    },

    getRemainingPiecesCount(gbs: GameBoardState, player: Player): number {
        const emptyCellsCount = gbs.cells.filter(cell => cell.var === CellEmptyVar).length

        if (player.var === gbs.current_player.var) {
            return Math.ceil(emptyCellsCount / 2)
        } else {
            return Math.floor(emptyCellsCount / 2)
        }
    },
}

export default Gbs
