use crate::utils::{board_state::BoardState, pattern::PatternState};

pub type Eval = f32;

pub fn eval(board_state: &BoardState) -> Eval {
    match board_state.state() {
        PatternState::Won(player) if player == board_state.turn() => {
            return  1000.0;
        },
        PatternState::Won(player) if player != board_state.turn() => {
            return -1000.0;
        },
        _ => (),
    }

    let mut eval = 0.0;

    eval += 1.0 * eval_terms::subboards_won(board_state);

    eval += 0.3 * eval_terms::subboards_almost_won(board_state);

    eval += 0.3 * eval_terms::subboards_doubly_almost_won(board_state);

    eval
}

mod eval_terms {
    use crate::{algorithms::minimax::eval::Eval, utils::board_state::BoardState};

    pub fn subboards_won(board_state: &BoardState) -> Eval {
        let subboard_pattern = board_state.subboard_pattern();

        let subboards_won  = subboard_pattern.spots(board_state.turn().to_piece());
        let subboards_lost = subboard_pattern.spots(board_state.turn().opposite().to_piece());

        subboards_won .len() as f32 -
        subboards_lost.len() as f32
    }

    pub fn subboards_almost_won(board_state: &BoardState) -> Eval {
        let mut count = 0.0;

        count += board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_active()
                    .map(|pattern| pattern.almost_won_by(board_state.turn()))
                    .unwrap_or(false)
            })
            .count() as f32;

        count -= board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_active()
                    .map(|pattern| pattern.almost_won_by(board_state.turn().opposite()))
                    .unwrap_or(false)
            })
            .count() as f32;

        count
    }

    pub fn subboards_doubly_almost_won(board_state: &BoardState) -> Eval {
        let mut count = 0.0;

        count += board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_active()
                    .map(|pattern| pattern.doubly_almost_won_by(board_state.turn()))
                    .unwrap_or(false)
            })
            .count() as f32;

        count -= board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_active()
                    .map(|pattern| pattern.doubly_almost_won_by(board_state.turn().opposite()))
                    .unwrap_or(false)
            })
            .count() as f32;

        count
    }
}
