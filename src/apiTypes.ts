/* eslint-disable linebreak-style */
// Auto-generated by api_typer

export type AppState = (
	{ var: "Title" } |
	{ var: "Game", vardata: GameBoardState }
)

export const AppStateTitleVar = "Title"
export const AppStateGameVar = "Game"


export interface GameBoardState {
	tick: number,
	player1_input: InputType,
	player2_input: InputType,
	cells: Array<Cell>,
	width: number,
	height: number,
	current_player: Player,
	winning_segment: Segment | null,
	player1_score: number,
	player2_score: number,
}

export type InputType = (
	{ var: "Local" } |
	{ var: "Bot" }
)

export const InputTypeLocalVar = "Local"
export const InputTypeBotVar = "Bot"


export type Cell = (
	{ var: "Empty" } |
	{ var: "Player1Piece" } |
	{ var: "Player2Piece" }
)

export const CellEmptyVar = "Empty"
export const CellPlayer1PieceVar = "Player1Piece"
export const CellPlayer2PieceVar = "Player2Piece"


export type Player = (
	{ var: "Player1" } |
	{ var: "Player2" }
)

export const PlayerPlayer1Var = "Player1"
export const PlayerPlayer2Var = "Player2"


export interface Segment {
	player: Player,
	points: Array<Point>,
}

export interface Point {
	x: number,
	y: number,
}
