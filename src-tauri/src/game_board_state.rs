use anyhow::anyhow;

pub use crate::api_types::{Cell, GameBoardState, InputType, Player, Point, Segment};
use crate::{ai::AiBrain, api_types::Difficulty};

impl From<Player> for Cell {
    fn from(player: Player) -> Self {
        match player {
            Player::Player1 => Cell::Player1Piece,
            Player::Player2 => Cell::Player2Piece,
        }
    }
}

impl GameBoardState {
    pub fn vs_p2() -> Self {
        GameBoardState::new(7, 6, InputType::Local).unwrap()
    }

    pub fn vs_bot(difficulty: Difficulty) -> Self {
        GameBoardState::new(7, 6, InputType::Bot { difficulty }).unwrap()
    }

    pub fn new(width: i32, height: i32, player2_input: InputType) -> anyhow::Result<Self> {
        Self::check_width_height(width, height)?;

        let size = (width * height) as usize;

        Ok(Self {
            tick: 0,
            player1_input: InputType::Local,
            player2_input,
            cells: vec![Cell::Empty; size],
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
        self.cells.fill(Cell::Empty);
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

    pub fn set(&mut self, row: i32, column: i32, value: Cell) -> anyhow::Result<()> {
        let i = self.coord_to_index(row, column)?;
        self.cells[i] = value;
        Ok(())
    }

    pub fn get(&self, row: i32, column: i32) -> anyhow::Result<Cell> {
        let i = self.coord_to_index(row, column)?;
        Ok(self.cells[i].clone())
    }

    pub fn is_set(&self, row: i32, column: i32) -> anyhow::Result<bool> {
        Ok(self.get(row, column)? != Cell::Empty)
    }

    pub fn put_piece_in_column(&mut self, column: i32) -> anyhow::Result<()> {
        if self.winning_segment.is_some() {
            return Err(anyhow!("Game is over"));
        }

        for r in (0..self.height).rev() {
            if !self.is_set(r, column)? {
                self.set(r, column, self.current_player.clone().into())?;

                self.winning_segment = self.find_winning_segment();

                match self.winning_segment {
                    None => {}
                    Some(Segment {
                        player: Player::Player1,
                        ..
                    }) => {
                        self.player1_score += 1;
                    }
                    Some(Segment {
                        player: Player::Player2,
                        ..
                    }) => {
                        self.player2_score += 1;
                    }
                }

                self.advance_turn();

                return Ok(());
            }
        }

        Err(anyhow!("Column is full"))
    }

    fn current_player_input(&self) -> &InputType {
        match self.current_player {
            Player::Player1 => &self.player1_input,
            Player::Player2 => &self.player2_input,
        }
    }

    fn advance_turn(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    pub fn take_turn_if_bot(&mut self, ai: &mut AiBrain) {
        if self.winning_segment.is_none() {
            match self.current_player_input() {
                InputType::Local => {}
                InputType::Bot { difficulty } => {
                    let c = ai.get_best_move(difficulty, self);
                    self.put_piece_in_column(c).ok();
                }
            }
        }
    }

    fn find_winning_segment(&self) -> Option<Segment> {
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

    fn check_if_segment_is_win(&self, r0: i32, c0: i32, dr: i32, dc: i32) -> Option<Segment> {
        const N: i32 = 4;

        let player = match self.get(r0, c0) {
            Ok(Cell::Empty) | Err(_) => {
                return None;
            }
            Ok(Cell::Player1Piece) => Player::Player1,
            Ok(Cell::Player2Piece) => Player::Player2,
        };

        let segment_iter = (0..N).map(|i| Point {
            x: c0 + dc * i,
            y: r0 + dr * i,
        });

        let cmp_player_piece: Cell = player.clone().into();

        // The last piece is more likely to be off the board, so checking it first is usually faster
        for Point { x, y } in segment_iter.clone().rev().take((N - 1) as usize) {
            if self.get(y, x).unwrap_or(Cell::Empty) != cmp_player_piece {
                return None;
            }
        }

        let segment = Segment {
            player,
            points: segment_iter.collect(),
        };

        println!("{:?}", segment);

        Some(segment)
    }

    pub fn step_tick(&mut self) {
        self.tick += 1;
    }
}
