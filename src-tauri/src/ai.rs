use float_ord::FloatOrd;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use crate::api_types::{Cell, GameBoardState, Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AiPlayer {
    Me,
    Them,
}

// impl AiPlayer {
//     fn next(self) -> Self {
//         match self {
//             AiPlayer::Me => AiPlayer::Them,
//             AiPlayer::Them => AiPlayer::Me,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq)]
enum AiWinState {
    Unknown(f32),
    Win,
    Lose,
}

impl AiWinState {
    fn score(&self) -> f32 {
        match self {
            AiWinState::Unknown(x) => *x,
            AiWinState::Win => 1.0,
            AiWinState::Lose => 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AiBoard {
    cells: Vec<Option<AiPlayer>>,
    width: i32,
    height: i32,
}

impl Hash for AiBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
    }
}

impl AiBoard {
    fn from_gameboard(gameboard: &GameBoardState) -> Self {
        let cells = gameboard
            .cells
            .iter()
            .map(|cell| match cell {
                Cell::Empty => None,
                Cell::Player1Piece => {
                    if gameboard.current_player == Player::Player1 {
                        Some(AiPlayer::Me)
                    } else {
                        Some(AiPlayer::Them)
                    }
                }
                Cell::Player2Piece => {
                    if gameboard.current_player == Player::Player2 {
                        Some(AiPlayer::Me)
                    } else {
                        Some(AiPlayer::Them)
                    }
                }
            })
            .collect();

        Self {
            cells,
            width: gameboard.width,
            height: gameboard.height,
        }
    }

    fn with_move(&self, player: AiPlayer, column: i32) -> Option<Self> {
        if column < 0 || column >= self.width {
            return None;
        }

        let mut next = self.clone();

        for row in (0..next.height).rev() {
            let cell = next.cells[(row * next.width + column) as usize];
            if cell.is_none() {
                next.cells[(row * next.width + column) as usize] = Some(player);
                return Some(next);
            }
        }

        None
    }

    fn rc_to_index(&self, row: i32, col: i32) -> Option<usize> {
        if row >= 0 && row < self.height && col >= 0 && col < self.width {
            Some((row * self.width + col) as usize)
        } else {
            None
        }
    }

    fn index_to_rc(&self, index: usize) -> (i32, i32) {
        let row = index as i32 / self.width;
        let col = index as i32 % self.width;
        (row, col)
    }

    // fn get(&self, row: i32, col: i32) -> Option<Option<AiPlayer>> {
    //     self.get_index(row, col).map(|i| self.cells[i])
    // }

    fn win_state(&self) -> AiWinState {
        const WIN_SEGMENT_LENGTH: i32 = 4;

        let mut used_cells = vec![false; self.cells.len()];
        let mut my_longest_segment = 0;
        let mut their_longest_segment = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            if !used_cells[i] {
                if let Some(player) = cell {
                    let (r, c) = self.index_to_rc(i);
                    for (dr, dc) in &[(0, 1), (1, 1), (1, 0), (1, -1)] {
                        let mut count = 0;
                        loop {
                            let i = match self.rc_to_index(r + dr * count, c + dc * count) {
                                Some(i) => i,
                                None => break,
                            };

                            if *cell != self.cells[i] {
                                break;
                            }

                            count += 1;

                            if count >= WIN_SEGMENT_LENGTH {
                                match player {
                                    AiPlayer::Me => return AiWinState::Win,
                                    AiPlayer::Them => return AiWinState::Lose,
                                }
                            }

                            match player {
                                AiPlayer::Me => my_longest_segment = my_longest_segment.max(count),
                                AiPlayer::Them => {
                                    their_longest_segment = their_longest_segment.max(count)
                                }
                            }

                            used_cells[i] = true;
                        }
                    }
                }
            }
        }

        let heuristic_score =
            ((my_longest_segment - their_longest_segment) as f32 / WIN_SEGMENT_LENGTH as f32 + 1.0)
                / 2.0;

        AiWinState::Unknown(heuristic_score)
    }
}

pub struct AiBrain {
    cache: HashMap<AiBoard, AiWinState>,
    look_ahead: usize,
}

impl AiBrain {
    pub fn new(look_ahead: usize) -> Self {
        Self {
            cache: Default::default(),
            look_ahead,
        }
    }

    pub fn get_best_move(&mut self, gameboard: &GameBoardState) -> i32 {
        let ai_board = AiBoard::from_gameboard(gameboard);

        let move_scores = (0..ai_board.width)
            .filter_map(|column| {
                ai_board
                    .with_move(AiPlayer::Me, column)
                    .map(|board| (column, self.get_score_cached(board)))
            })
            .collect::<Vec<_>>();

        println!("Scores: {:?}", move_scores);

        move_scores
            .into_iter()
            .max_by_key(|(_, score)| FloatOrd(*score))
            .map(|(column, _)| column)
            .unwrap_or(ai_board.width / 2)
    }

    fn get_score_cached(&mut self, board: AiBoard) -> f32 {
        if let Some(win_state) = self.cache.get(&board) {
            return win_state.score();
        }

        let win_state = self.get_win_state(&board, self.look_ahead);

        self.cache.insert(board, win_state);

        win_state.score()
    }

    fn get_win_state(&mut self, board: &AiBoard, look_ahead: usize) -> AiWinState {
        let win_state = board.win_state();

        let win_state = match win_state {
            AiWinState::Unknown(h) => {
                let their_moves = (0..board.width)
                    .filter_map(|column| board.with_move(AiPlayer::Them, column))
                    .collect::<Vec<_>>();

                let lose_any = their_moves
                    .iter()
                    .any(|b| b.win_state() == AiWinState::Lose);

                if lose_any {
                    AiWinState::Lose
                } else if look_ahead > 0 {
                    let my_moves_scores = their_moves
                        .iter()
                        .flat_map(|board| {
                            (0..board.width)
                                .filter_map(|column| board.with_move(AiPlayer::Me, column))
                                .collect::<Vec<_>>()
                        })
                        .map(|board| self.get_win_state(&board, look_ahead - 1).score())
                        .collect::<Vec<_>>();

                    AiWinState::Unknown(
                        my_moves_scores.iter().sum::<f32>() / my_moves_scores.len() as f32,
                    )
                } else {
                    AiWinState::Unknown(h)
                }
            }
            win_state => win_state,
        };

        win_state
    }
}
