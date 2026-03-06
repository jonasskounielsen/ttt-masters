use crate::{algorithms::minimax::eval::{Eval, dbg_print_eval_breakdown, eval}, utils::{Move, board_state::BoardState, pattern::PatternState}};

mod eval;

const SEARCH_DEPTH: u32 = 1;

pub fn minimax(board_state: &BoardState) -> Move<'_> {
    let eligible_moves = board_state.eligible_moves();

    let best_move_index = eligible_moves
        .iter()
        .enumerate()
        .map(|(index, move_)| {
            let eval = minimax_inner(&board_state.do_move(*move_), SEARCH_DEPTH - 1, false);
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

    eligible_moves[best_move_index]
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
    board_state.dbg_print();
    
    let eligible_moves = board_state.eligible_moves();

    eligible_moves
        .iter()
        .map(|move_| minimax_inner(&board_state.do_move(*move_), depth_plies - 1, !own_turn))
        .reduce(|best_move, move_| {
            if own_turn { // Assume that the enemy plays optimally.
                best_move.max(move_)
            } else {
                best_move.min(move_)
            }
        }).expect("no eligible move")
}

#[allow(unused)]
pub fn dbg_print_moves(board_state: &BoardState) {
    let eligible_moves = board_state.eligible_moves();

    eligible_moves
        .iter()
        .enumerate()
        .for_each(|(index, move_)| {
            dbg_print_eval_breakdown(&board_state.do_move(*move_), *move_, index);
        });
}
