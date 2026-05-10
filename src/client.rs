use crate::utils::{Player, board_state::BoardState};

#[derive(Debug)]
pub struct Game {
    board_state: BoardState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board_state: BoardState::new_empty(Player::Cross),
        }
    }
}
