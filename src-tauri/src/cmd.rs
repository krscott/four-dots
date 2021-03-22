use serde::Deserialize;

use crate::state::State;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Nop,
    PutPieceInColumn { column: i32 },
    ClearBoard,
}

impl Cmd {
    pub fn handle(&self, state: &mut State) -> anyhow::Result<()> {
        state.step_tick();

        match self {
            Cmd::Nop => {}
            Cmd::PutPieceInColumn { column } => {
                state.put_piece_in_column(*column)?;
            }
            Cmd::ClearBoard => {
                state.clear();
            }
        }

        Ok(())
    }
}
