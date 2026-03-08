use crate::{algorithms::minimax::{eval::{Eval, dbg_print_eval_breakdown, eval}, transposition_table::TranspositionTable}, utils::{Move, board_state::BoardState, pattern::PatternState}};

mod eval;
mod transposition_table;

const SEARCH_DEPTH_PLIES: u32 = 3;

pub fn minimax(board_state: &BoardState) -> Move<'_> {
    let mut transposition_table = TranspositionTable::new();

    let eligible_moves = board_state.eligible_moves();

    *eligible_moves
        .iter()
        .map(|move_| {
            let eval = minimax_inner(&board_state.do_move(*move_), &mut transposition_table, SEARCH_DEPTH_PLIES - 1, false);
            (eval, move_)
        })
        .reduce(|best_move, move_| {
            if best_move.0 > move_.0 {
                best_move
            } else {
                move_
            }
        })
        .expect("no eligible move")
        .1
}

fn minimax_inner(
    board_state: &BoardState,
    transposition_table: &mut TranspositionTable,
    depth: u32,
    own_turn: bool,
) -> Eval {
    if let Some(eval) = transposition_table.get(board_state, depth) {
        return eval;
    }

    if depth == 0 || matches!(board_state.state(), PatternState::Won(_)) {
        let eval = eval(board_state);
        transposition_table.set(board_state, depth, eval);
        return if own_turn {
             eval
        } else {
            -eval
        };
    }
    
    let eligible_moves = board_state.eligible_moves();

    let eval = eligible_moves
        .iter()
        .map(|move_|
            minimax_inner(&board_state.do_move(*move_), transposition_table, depth - 1, !own_turn)
        )
        .reduce(|best_move, move_| {
            if own_turn { // Assume that the enemy plays optimally.
                best_move.max(move_)
            } else {
                best_move.min(move_)
            }
        })
        .expect("no eligible move");

    transposition_table.set(board_state, depth, eval);
    eval
}

#[allow(unused)]
pub fn dbg_print_moves(board_state: &BoardState) {
    eprintln!("{:<51}  gmalw |  sbw  | sbwpl | sbalw | sbdaw | pcpl  | apcpl", "");

    let eligible_moves = board_state.eligible_moves();

    eligible_moves
        .iter()
        .enumerate()
        .for_each(|(index, move_)| {
            dbg_print_eval_breakdown(&board_state.do_move(*move_), *move_, index);
        });
}
