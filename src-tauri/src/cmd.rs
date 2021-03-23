use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Nop,
    PutPieceInColumn { column: i32 },
    ClearBoard,
}

impl Cmd {
    pub fn handle(&self, state: &mut AppState) -> anyhow::Result<()> {
        match state {
            AppState::Title => {}
            AppState::Game(game_board_state) => {
                game_board_state.step_tick();

                match self {
                    Cmd::Nop => {}
                    Cmd::PutPieceInColumn { column } => {
                        game_board_state.put_piece_in_column(*column)?;
                    }
                    Cmd::ClearBoard => {
                        game_board_state.clear();
                    }
                }
            }
        }

        Ok(())
    }
}
