use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use validator::{Validate, ValidationError};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum MaybePlayer {
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

impl From<Player> for MaybePlayer {
    fn from(player: Player) -> Self {
        match player {
            Player::Player1 => MaybePlayer::Player1Piece,
            Player::Player2 => MaybePlayer::Player2Piece,
        }
    }
}

fn validate_state(state: &GameBoardState) -> Result<(), ValidationError> {
    if state.cells.len() as i32 != state.width + state.height {
        Err(ValidationError::new(
            "cells_length_does_not_match_width_height",
        ))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_state"))]
pub struct GameBoardState {
    tick: usize,
    #[validate(length(min = 1))]
    cells: Vec<MaybePlayer>,
    #[validate(range(min = 1))]
    width: i32,
    #[validate(range(min = 1))]
    height: i32,
    current_player: Player,
    winning_segment: Option<(Player, Vec<(i32, i32)>)>,
    player1_score: u16,
    player2_score: u16,
}

impl GameBoardState {
    pub fn new(width: i32, height: i32) -> anyhow::Result<Self> {
        Self::check_width_height(width, height)?;

        let size = (width * height) as usize;

        Ok(Self {
            tick: 0,
            cells: vec![MaybePlayer::None; size],
            width,
            height,
            current_player: Player::Player1,
            winning_segment: None,
            player1_score: 0,
            player2_score: 0,
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
        self.cells.fill(MaybePlayer::None);
        self.winning_segment = None;
    }

    // pub fn width(&self) -> i32 {
    //     self.width
    // }

    // pub fn height(&self) -> i32 {
    //     self.height
    // }

    // pub fn current_player(&self) -> Player {
    //     self.current_player
    // }

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

    pub fn set(&mut self, row: i32, column: i32, value: MaybePlayer) -> anyhow::Result<()> {
        let i = self.coord_to_index(row, column)?;
        self.cells[i] = value;
        Ok(())
    }

    pub fn get(&self, row: i32, column: i32) -> anyhow::Result<MaybePlayer> {
        let i = self.coord_to_index(row, column)?;
        Ok(self.cells[i])
    }

    pub fn is_set(&self, row: i32, column: i32) -> anyhow::Result<bool> {
        Ok(self.get(row, column)? != MaybePlayer::None)
    }

    pub fn put_piece_in_column(&mut self, column: i32) -> anyhow::Result<()> {
        if self.winning_segment.is_some() {
            return Err(anyhow!("Game is over"));
        }

        for r in (0..self.height).rev() {
            if !self.is_set(r, column)? {
                self.set(r, column, self.current_player.into())?;

                self.winning_segment = self.find_winning_segment();

                match self.winning_segment {
                    None => {}
                    Some((Player::Player1, _)) => {
                        self.player1_score += 1;
                    }
                    Some((Player::Player2, _)) => {
                        self.player2_score += 1;
                    }
                }

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

    fn find_winning_segment(&self) -> Option<(Player, Vec<(i32, i32)>)> {
        for r0 in 0..self.height {
            for c0 in 0..self.width {
                for (dr, dc) in &[(0, 1), (1, 1), (1, 0), (1, -1)] {
                    match self.check_if_segment_is_win(r0, c0, *dr, *dc) {
                        Some(seg) => return Some(seg),
                        None => {}
                    }
                }
            }
        }

        None
    }

    fn check_if_segment_is_win(
        &self,
        r0: i32,
        c0: i32,
        dr: i32,
        dc: i32,
    ) -> Option<(Player, Vec<(i32, i32)>)> {
        const N: i32 = 4;

        let player = match self.get(r0, c0) {
            Ok(MaybePlayer::None) | Err(_) => {
                return None;
            }
            Ok(MaybePlayer::Player1Piece) => Player::Player1,
            Ok(MaybePlayer::Player2Piece) => Player::Player2,
        };

        let segment_iter = (0..N).map(|i| (r0 + dr * i, c0 + dc * i));

        let cmp_player_piece = player.into();

        // The last piece is more likely to be off the board, so checking it first is usually faster
        for (r, c) in segment_iter.clone().rev().take((N - 1) as usize) {
            if self.get(r, c).unwrap_or(MaybePlayer::None) != cmp_player_piece {
                return None;
            }
        }

        let segment = (player, segment_iter.collect());

        println!("{:?}", segment);

        Some(segment)
    }

    pub fn step_tick(&mut self) {
        self.tick += 1;
    }
}
