use serde::{Deserialize, Serialize};

use crate::game_board_state::GameBoardState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", content = "data", rename_all = "camelCase")]
pub enum AppState {
    Title,
    Game(GameBoardState),
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Title
        // AppState::Game(GameBoardState::new(7, 6).unwrap())
    }
}
