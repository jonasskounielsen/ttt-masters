use crate::utils::{BoardState, Move};

pub fn greedy(board_state: BoardState) -> Move {
    let possible_moves = board_state.eligible_moves();

    if let Some(winning_move) = possible_moves.iter().find(
        |move_| move_
    ) {
        return Move::new(winning_move);
    }

    ()
}