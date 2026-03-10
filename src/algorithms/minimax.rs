use crate::{algorithms::minimax::{eval::{EVAL_LOST, EVAL_WON, Eval, dbg_print_eval_breakdown, eval}, transposition_table::TranspositionTable}, utils::{Move, board_state::BoardState, pattern::PatternState}};

mod eval;
mod transposition_table;

const SEARCH_DEPTH_PLIES: u32 = 3;

pub fn minimax(board_state: &BoardState) -> Move<'_> {
    let mut transposition_table = TranspositionTable::new();

    minimax_inner(
        board_state, &mut transposition_table,
        0, true,
        EVAL_LOST - 1.0, EVAL_WON + 1.0,
    );

    let eligible_moves = board_state.eligible_moves();

    *eligible_moves
        .iter()
        .filter_map(|move_| {
            let eval = transposition_table.get(&board_state.do_move(*move_), 1);
            eval.map(|eval| (eval, move_))
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
    board_state:         &BoardState,
    transposition_table: &mut TranspositionTable,
    depth:               u32,
    own_turn:            bool,
    mut alpha:           Eval,
    mut beta:            Eval,
) -> Eval {
    if let Some(eval) = transposition_table.get(board_state, depth) {
        return eval;
    }

    if depth == SEARCH_DEPTH_PLIES || matches!(board_state.state(), PatternState::Won(_)) {
        let eval = if own_turn {
             eval(board_state)
        } else {
            -eval(board_state)
        };
        transposition_table.set(board_state, depth, eval);
        return eval;
    }
    
    let eligible_moves = board_state.eligible_moves();
    if eligible_moves.is_empty() {
        panic!("no eligible move");
    }

    let mut best_eval = if own_turn {
        EVAL_LOST
    } else {
        EVAL_WON
    };
    for move_ in eligible_moves {
        let eval = minimax_inner(
            &board_state.do_move(move_), transposition_table,
            depth + 1, !own_turn,
            alpha, beta,
        );
        if own_turn {
            best_eval = best_eval.max(eval);
            if best_eval >= beta { // Beta cutoff.
                return best_eval;
            }
            alpha = alpha.max(best_eval);
        } else {
            best_eval = best_eval.min(eval);
            if best_eval <= alpha { // Alpha cutoff.
                return best_eval;
            }
            beta = beta.min(eval);
        }
    }

    dbg!(depth);
    transposition_table.set(board_state, depth, best_eval);
    best_eval
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
