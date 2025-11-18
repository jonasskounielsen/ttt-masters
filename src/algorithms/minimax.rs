use crate::utils::{Move, board_state::BoardState, debug::dbg_MoveList, pattern::PatternState};

const DEPTH_PLIES: u32 = 3;

pub fn minimax(board_state: &BoardState) -> Move<'_> {
    let eligible_moves = board_state.eligible_moves();

    eligible_moves.dbg_print();
    let best_move_index = eligible_moves
        .iter()
        .map(|move_| minimax_inner(&board_state.do_move(*move_), DEPTH_PLIES - 1, false))
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
    
    let eligible_moves = board_state.eligible_moves();

    eligible_moves
        .iter()
        .map(|move_| minimax_inner(&board_state.do_move(*move_), depth_plies - 1, !own_turn))
        .reduce(|best_move, move_| {
            best_move.max(move_)
        }).expect("no eligible move")
}

type Eval = i32;

fn eval(board_state: &BoardState) -> Eval {
    match board_state.state() {
        PatternState::Won(player) if player == board_state.turn() => {
            return  1000;
        },
        PatternState::Won(player) if player != board_state.turn() => {
            return -1000;
        },
        _ => (),
    }

    let subboard_pattern = board_state.subboard_pattern();

    let subboards_won  = subboard_pattern.spots(board_state.turn().to_piece());
    let subboards_lost = subboard_pattern.spots(board_state.turn().opposite().to_piece());

    <usize as TryInto<i32>>::try_into(subboards_won.len()).unwrap() - <usize as TryInto<i32>>::try_into(subboards_lost.len()).unwrap()
}
