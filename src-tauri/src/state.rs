use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use validator::{Validate, ValidationError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Cell {
    None = 0,
    Player1Piece = 1,
    Player2Piece = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Player {
    Player1 = 1,
    Player2 = 2,
}

impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        match player {
            Player::Player1 => Cell::Player1Piece,
            Player::Player2 => Cell::Player2Piece,
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_state"))]
pub struct State {
    #[validate(length(min = 1))]
    cells: Vec<Cell>,
    #[validate(range(min = 1))]
    width: i32,
    #[validate(range(min = 1))]
    height: i32,
    current_player: Player,
}

fn validate_state(state: &State) -> Result<(), ValidationError> {
    if state.cells.len() as i32 != state.width + state.height {
        Err(ValidationError::new(
            "cells_length_does_not_match_width_height",
        ))
    } else {
        Ok(())
    }
}

impl State {
    pub fn new(width: i32, height: i32) -> anyhow::Result<Self> {
        Self::check_width_height(width, height)?;

        let size = (width * height) as usize;

        Ok(Self {
            cells: vec![Cell::None; size],
            width,
            height,
            current_player: Player::Player1,
        })
    }

    fn check_width_height(width: i32, height: i32) -> anyhow::Result<()> {
        if width <= 0 {
            return Err(anyhow!("width must be greater than 0"));
        }
        if height <= 0 {
            return Err(anyhow!("height must be greater than 0"));
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::None);
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn is_valid_coord(&self, row: i32, column: i32) -> bool {
        row >= 0 && row < self.height && column >= 0 && column < self.width
    }

    fn coord_to_index(&self, row: i32, column: i32) -> anyhow::Result<usize> {
        if self.is_valid_coord(row, column) {
            Ok((row * self.width + column) as usize)
        } else {
            Err(anyhow!("Invalid coordinate"))
        }
    }

    pub fn set(&mut self, row: i32, column: i32, value: Cell) -> anyhow::Result<()> {
        let i = self.coord_to_index(row, column)?;
        self.cells[i] = value;
        Ok(())
    }

    pub fn get(&self, row: i32, column: i32) -> anyhow::Result<Cell> {
        let i = self.coord_to_index(row, column)?;
        Ok(self.cells[i])
    }

    pub fn is_set(&self, row: i32, column: i32) -> anyhow::Result<bool> {
        Ok(self.get(row, column)? != Cell::None)
    }

    pub fn put_piece_in_column(&mut self, column: i32) -> anyhow::Result<()> {
        for r in (0..self.height).rev() {
            if !self.is_set(r, column)? {
                self.set(r, column, self.current_player.into())?;
                self.advance_turn();
                return Ok(());
            }
        }

        Err(anyhow!("Column is full"))
    }

    fn advance_turn(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }
}
