use crate::utils::{board_state::BoardState, debug::dbg_MoveList, Centeredness, Move, Place};

// "greedy" chooses, in order of priority:
// * A move that wins.
// * A move that blocks the enemy from winning.
// * A move that wins a subboard.
// * A move that blocks the enemy from winning a subboard.
// * The move that takes the square with the highest centeredness.
//
// When the most preferable move is available on multiple subboards, they are prioritised by centeredness.
// When multiple moves fit the criteria for preferable, the first one is chosen.
// Possible centeredness values are, from higher to lower:
// * The center of a pattern.
// * The corner of a pattern.
// * The edge of a pattern.

pub fn greedy(board_state: &BoardState) -> Move<'_> {
    let eligible_moves = board_state.eligible_moves();
    eligible_moves.dbg_print();
    
    let first_winning_move = eligible_moves.iter().find(|move_| {
        board_state.subboard_pattern().wins(move_.subboard(), board_state.turn())
    });

    if let Some(winning_move) = first_winning_move {
        return *winning_move;
    }

    let first_win_blocking_move = eligible_moves.iter().find(|move_| {
        board_state.subboard_pattern().blocks(move_.subboard(), board_state.turn())
    });

    if let Some(blocking_move) = first_win_blocking_move {
        return *blocking_move;
    }
    
    let best_subboard_winning_move = best_subboard_winning_move(board_state, &eligible_moves);

    if let Some(move_) = best_subboard_winning_move {
        return move_;
    }

    let best_subboard_blocking_move = best_subboard_blocking_move(board_state, &eligible_moves);

    if let Some(move_) = best_subboard_blocking_move {
        return move_;
    }

    let best_centermost_square_move = best_centermost_square_move(&eligible_moves);

    if let Some(move_) = best_centermost_square_move {
        return move_;
    }
    
    panic!("no eligible moves");
}

fn best_subboard_winning_move<'a>(board_state: &'a BoardState, eligible_moves: &Box<[Move<'a>]>) -> Option<Move<'a>> {
    let mut subboard_winning_moves: Vec<_> = eligible_moves
        .iter()
        .filter(|move_| {
            let Some(pattern) = board_state.pattern_if_active(move_.subboard()) else {
                panic!("move points to inactive subboard");
            };
            pattern.wins(move_.square(), board_state.turn())
        })
        .collect();
    
    subboard_winning_moves.sort_by(|move1, move2| {
        cmp_centeredness(move1.subboard(), move2.subboard())
    });
    
    subboard_winning_moves.get(0).map(|move_| **move_)
}

fn best_subboard_blocking_move<'a>(board_state: &'a BoardState, eligible_moves: &Box<[Move<'a>]>) -> Option<Move<'a>> {
    let mut subboard_winning_moves: Vec<_> = eligible_moves
        .iter()
        .filter(|move_| {
            let Some(pattern) = board_state.pattern_if_active(move_.subboard()) else {
                panic!("move points to inactive subboard");
            };
            pattern.blocks(move_.square(), board_state.turn())
        })
        .collect();
    
    subboard_winning_moves.sort_by(|move1, move2| {
        cmp_centeredness(move1.subboard(), move2.subboard())
    });
    
    subboard_winning_moves.get(0).map(|move_| **move_)
}

fn best_centermost_square_move<'a>(eligible_moves: &Box<[Move<'a>]>) -> Option<Move<'a>> {
    let mut moves = eligible_moves.clone();

    moves.sort_by(|move1, move2| {
        cmp_centeredness(move2.square(), move1.square())
    });

    moves.sort_by(|move1, move2| {
        cmp_centeredness(move2.subboard(), move1.subboard())
    });

    moves.get(0).map(|move_| *move_)
}

fn cmp_centeredness(place1: Place, place2: Place) -> std::cmp::Ordering {
    match (place1.centeredness(), place2.centeredness()) {
        (Centeredness::Center, Centeredness::Center) => std::cmp::Ordering::Equal,
        (Centeredness::Center, _)                    => std::cmp::Ordering::Greater,
        (Centeredness::Corner, Centeredness::Corner) => std::cmp::Ordering::Equal,
        (Centeredness::Corner, Centeredness::Edge)   => std::cmp::Ordering::Greater,
        (Centeredness::Edge,   Centeredness::Edge)   => std::cmp::Ordering::Equal,
        _                                            => std::cmp::Ordering::Less,
    }
}