pub use crate::api_types::AppState;

impl Default for AppState {
    fn default() -> Self {
        AppState::Title
        // AppState::Game(GameBoardState::new(7, 6).unwrap())
    }
}
