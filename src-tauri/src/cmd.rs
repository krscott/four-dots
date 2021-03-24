use serde::Deserialize;

use crate::{app_state::AppState, game_board_state::GameBoardState};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Nop,
    Start1P,
    Start2P,
    PutPieceInColumn { column: i32 },
    ClearBoard,
    ReturnToTitle,
}

impl Cmd {
    pub fn handle(&self, state: &mut AppState) -> anyhow::Result<()> {
        match state {
            AppState::Title => {
                match self {
                    Cmd::Nop => {}
                    Cmd::Start1P | Cmd::Start2P => {
                        *state = AppState::Game(GameBoardState::default());
                    }
                    Cmd::PutPieceInColumn { .. } => {}
                    Cmd::ClearBoard => {}
                    Cmd::ReturnToTitle => {}
                };
            }
            AppState::Game(game_board_state) => {
                game_board_state.step_tick();

                match self {
                    Cmd::Nop => {}
                    Cmd::Start1P | Cmd::Start2P => game_board_state.clear(),
                    Cmd::PutPieceInColumn { column } => {
                        game_board_state.put_piece_in_column(*column)?;
                    }
                    Cmd::ClearBoard => {
                        game_board_state.clear();
                    }
                    Cmd::ReturnToTitle => *state = AppState::Title,
                }
            }
        }

        Ok(())
    }
}
