use crate::utils::{board_state::BoardState, pattern::PatternState, Move};

// "greedy" chooses, in order of priority:
// * A move that wins.
// * A move that wins a subboard, subboards being prioritised the same as squares.
// * A move that blocks the enemy from winning a subboard, subboards being prioritised the same as squares.
// * A move that takes the center of a subboard.
// * A move that takes a square adjacent to the center of a subboard, eg. not a corner.
// * Any move.

pub fn greedy(board_state: BoardState) -> Move {
    let possible_moves = board_state.eligible_moves();

    if let Some(winning_move) = possible_moves.iter().find(|move_| {
        board_state.do_move(**move_).state() == board_state.turn()
    }
    ) {
        return *winning_move;
    }
    
    let 

    possible_moves[0]
}