use crate::{algorithms::minimax::eval::{EVAL_LOST, EVAL_WON, Eval, eval, eval_terms}, utils::{Move, board_state::BoardState}};

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

pub fn dbg_print_eval_breakdown(board_state: &BoardState, move_: Move, index: usize) {
    // Negate eval to present from other player's perspective.
    eprintln!(
        "{:>2}: eval: {}, {} ({}, {}, {}, {}, {}, {}, {})",
        index,
        format_eval(-eval(board_state)),
        move_.dbg_to_string(),
        format_eval(-eval_terms::eval_game_almost_won       (board_state)),
        format_eval(-eval_terms::eval_subboards_won         (board_state)),
        format_eval(-eval_terms::eval_subboards_won_places  (board_state)),
        format_eval(-eval_terms::subboards_almost_won       (board_state)),
        format_eval(-eval_terms::subboards_doubly_almost_won(board_state)),
        format_eval(-eval_terms::eval_piece_places          (board_state)),
        format_eval(-eval_terms::eval_active_subboard_pieces(board_state)),
    );
}

fn format_eval(eval: Eval) -> String {
    if eval == EVAL_WON || eval == EVAL_LOST {
        return format!(" {:+>5}", eval);
    }
    format!("{:+>6.3}", eval)
}
