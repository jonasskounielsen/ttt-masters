use std::{sync::mpsc, thread, time::{Duration, Instant}};

use crate::{algorithms::minimax::{eval::{EVAL_LOST, EVAL_WON, Eval, dbg_print_eval_breakdown, eval}, transposition_table::{TranspositionTable, TranspositionTableResponse}}, utils::{Move, board_state::BoardState, pattern::PatternState}};

mod eval;
mod transposition_table;

const MAX_DEPTH_PLIES: u32 = 99;
const MAX_SEARCH_TIME_MILLIS: u64 = 1_000;

enum Message {
    BestYetMove(Move),
    SearchTerminatedMove(Move),
    TimeUp,
}

pub fn minimax(board_state: &BoardState) -> Move {
    let mut transposition_table = TranspositionTable::new();

    let (tx, rx) = mpsc::channel();

    let tx_minimax = tx.clone();
    let board_state = *board_state;
    let test = Instant::now();
    thread::spawn(move || {
        let mut depth = 1;
        loop {
            minimax_inner(
                &board_state, &mut transposition_table,
                depth, true,
                EVAL_LOST - 1.0, EVAL_WON + 1.0,
            );
            let TranspositionTableResponse::PresentHighDepth {
                best_move: Some(move_), ..
            } = transposition_table.get(&board_state, 0) else {
                panic!("no eligible move");
            };
            dbg!(depth);
            dbg!(test.elapsed());
            if depth == MAX_DEPTH_PLIES {
                tx_minimax.send(Message::SearchTerminatedMove(move_)).expect("failed to send max depth move message to main thread");
                break;
            }
            tx_minimax.send(Message::BestYetMove(move_)).expect("failed to send best yet move message to main thread");
            depth += 1;
        }
    });

    let tx_time = tx.clone();
    let start_instant = Instant::now();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(10));
            let elapsed = start_instant.elapsed();
            if elapsed > Duration::from_millis(MAX_SEARCH_TIME_MILLIS) {
                tx_time.send(Message::TimeUp).expect("failed to send time up message to main thread");
            }
        }
    });

    let mut best_move_yet = None;
    for message in rx.iter() {
        match message {
            Message::BestYetMove(move_) => {
                best_move_yet = Some(move_);
            },
            Message::SearchTerminatedMove(move_) => {
                return move_;
            },
            Message::TimeUp => {
                return best_move_yet.expect("depth 1 search ran out of time");
            },
        }
    }
    unreachable!();
}

fn minimax_inner (
    board_state:         &BoardState,
    transposition_table: &mut TranspositionTable,
    depth:               u32,
    own_turn:            bool,
    mut alpha:           Eval,
    mut beta:            Eval,
) -> Eval {
    let transposition_table_response = transposition_table.get(board_state, depth);
    
    if let TranspositionTableResponse::PresentHighDepth { eval, .. } = transposition_table_response {
        return eval;
    }

    if depth == 0 || matches!(board_state.state(), PatternState::Won(_)) {
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

    let mut sorted_moves = Vec::new();
    if let TranspositionTableResponse::PresentLowDepth {
        eval: _eval, best_move: Some(best_move)
    } = transposition_table_response {
        sorted_moves.push(best_move);
        for move_ in &eligible_moves {
            if *move_ == best_move {
                continue;
            }
            sorted_moves.push(*move_);
        }
    }
    sorted_moves.extend_from_slice(&eligible_moves);

    let mut best_eval = if own_turn {
        EVAL_LOST
    } else {
        EVAL_WON
    };
    let mut best_move = sorted_moves[0]; // Will always be overwritten.
    for move_ in sorted_moves {
        let eval = minimax_inner(
            &board_state.do_move(move_), transposition_table,
            depth - 1, !own_turn,
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
