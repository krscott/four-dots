// Auto-generated by api_typer

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "var", content = "vardata")]
pub enum AppState {
	Title,
	Game(GameBoardState),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GameBoardState {
	pub tick: u32,
	pub player1_input: InputType,
	pub player2_input: InputType,
	pub cells: Vec<Cell>,
	pub width: i32,
	pub height: i32,
	pub current_player: Player,
	pub winning_segment: Option<Segment>,
	pub player1_score: i32,
	pub player2_score: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "var", content = "vardata")]
pub enum InputType {
	Local,
	Bot,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "var", content = "vardata")]
pub enum Cell {
	Empty,
	Player1Piece,
	Player2Piece,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "var", content = "vardata")]
pub enum Player {
	Player1,
	Player2,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Segment {
	pub player: Player,
	pub points: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}