use anyhow::anyhow;
use serde::Deserialize;

use crate::{ai::AiBrain, app_state::AppState, game_board_state::GameBoardState};

#[derive(Debug, Clone, Deserialize)]
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
    pub fn handle(&self, state: &mut AppState, ai: &mut AiBrain) -> anyhow::Result<()> {
        match state {
            AppState::Title => {
                match self {
                    Cmd::Nop => {}
                    Cmd::Start1P => {
                        *state = AppState::Game(GameBoardState::vs_bot());
                    }
                    Cmd::Start2P => {
                        *state = AppState::Game(GameBoardState::vs_p2());
                    }
                    Cmd::PutPieceInColumn { .. } | Cmd::ClearBoard | Cmd::ReturnToTitle => {
                        return Err(anyhow!(
                            "Unexpected command '{:?}' in state '{:?}'",
                            self,
                            state
                        ));
                    }
                };
            }
            AppState::Game(game_board_state) => {
                game_board_state.step_tick();

                match self {
                    Cmd::Nop => {}
                    Cmd::Start1P | Cmd::Start2P => {
                        return Err(anyhow!(
                            "Unexpected command '{:?}' in state '{:?}'",
                            self,
                            state
                        ));
                    }
                    Cmd::PutPieceInColumn { column } => {
                        game_board_state.put_piece_in_column(*column)?;
                        game_board_state.take_turn_if_bot(ai);
                    }
                    Cmd::ClearBoard => {
                        game_board_state.clear();
                        game_board_state.take_turn_if_bot(ai);
                    }
                    Cmd::ReturnToTitle => *state = AppState::Title,
                }
            }
        }

        Ok(())
    }
}
