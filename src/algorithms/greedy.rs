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
    
    let first_winning_move = possible_moves.iter().find(|move_| {
        board_state.do_move(**move_).state() == PatternState::Won(board_state.turn())
    });

    if let Some(winning_move) = first_winning_move {
        return *winning_move;
    }
    
    // let subboard_winnign_moves = possible_moves. 

    possible_moves[0]
}