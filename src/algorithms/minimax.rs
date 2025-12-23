use crate::utils::{Move, board_state::BoardState, debug::dbg_MoveList, pattern::PatternState};
use eval::{Eval, eval};

mod eval;

const SEARCH_DEPTH: u32 = 3;

pub fn minimax(board_state: &BoardState) -> Move<'_> {
    let eligible_moves = board_state.eligible_moves();

    eligible_moves.dbg_print();
    let best_move_index = eligible_moves
        .iter()
        .map(|move_| minimax_inner(&board_state.do_move(*move_), SEARCH_DEPTH - 1, false))
        .enumerate()
        .map(|(index, eval)| {
            eprintln!("{:>2}: {:>4}, {}", index, eval, eligible_moves[index].dbg_to_string());
            (eval, index)
        })
        .reduce(|best_eval, eval| {
            if best_eval.0 > eval.0 {
                best_eval
            } else {
                eval
            }
        })
        .expect("no eligible move")
        .1;

    dbg!(eligible_moves[best_move_index])
}

fn minimax_inner(board_state: &BoardState, depth_plies: u32, own_turn: bool) -> Eval {
    if depth_plies == 0 || matches!(board_state.state(), PatternState::Won(_)) {
        let eval = eval(board_state);
        return if own_turn {
            eval
        } else {
            -eval
        };
    }
    
    let eligible_moves = board_state.eligible_moves();

    eligible_moves
        .iter()
        .map(|move_| minimax_inner(&board_state.do_move(*move_), depth_plies - 1, !own_turn))
        .reduce(|best_move, move_| {
            if own_turn {
                best_move.max(move_)
            } else {
                best_move.min(move_)
            }
        }).expect("no eligible move")
}
