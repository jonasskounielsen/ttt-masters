use crate::{algorithms::minimax::{eval::{EVAL_LOST, EVAL_WON, Eval, dbg_print_eval_breakdown, eval}, transposition_table::{TranspositionTable, TranspositionTableResponse}}, utils::{Move, board_state::BoardState, pattern::PatternState}};

mod eval;
mod transposition_table;

const SEARCH_DEPTH_PLIES: u32 = 3;

pub fn minimax(board_state: &BoardState) -> Move {
    let mut transposition_table = TranspositionTable::new();

    minimax_inner(
        board_state, &mut transposition_table,
        0, true,
        EVAL_LOST - 1.0, EVAL_WON + 1.0,
    );

    dbg!(transposition_table.get(board_state, 0));
    let TranspositionTableResponse::PresentLowDepth {
        best_move: Some(move_), ..
    } = transposition_table.get(board_state, 0) else {
        panic!("no eligible move");
    };
    move_
}

fn minimax_inner (
    board_state:         &BoardState,
    transposition_table: &mut TranspositionTable,
    depth:               u32,
    own_turn:            bool,
    mut alpha:           Eval,
    mut beta:            Eval,
) -> Eval {
    if let TranspositionTableResponse::PresentLowDepth { eval, .. } = transposition_table.get(board_state, depth) {
        return eval;
    }

    if depth == SEARCH_DEPTH_PLIES || matches!(board_state.state(), PatternState::Won(_)) {
        let eval = if own_turn {
             eval(board_state)
        } else {
            -eval(board_state)
        };
        transposition_table.set(board_state, depth, eval, None);
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
    let mut best_move = eligible_moves[0]; // Will always be overwritten.
    for move_ in eligible_moves {
        let eval = minimax_inner(
            &board_state.do_move(move_), transposition_table,
            depth + 1, !own_turn,
            alpha, beta,
        );
        if own_turn {
            if eval > best_eval {
                best_eval = eval;
                best_move = move_;
            }
            if best_eval >= beta { // Beta cutoff.
                return best_eval;
            }
            alpha = alpha.max(best_eval);
        } else {
            if eval < best_eval {
                best_eval = eval;
                best_move = move_;
            }
            if best_eval <= alpha { // Alpha cutoff.
                return best_eval;
            }
            beta = beta.min(eval);
        }
    }

    dbg!(best_move);
    transposition_table.set(board_state, depth, best_eval, Some(best_move));
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
