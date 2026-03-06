use crate::utils::{Centeredness, Move, Place, board_state::BoardState, pattern::PatternState};

pub type Eval = f32;

const SUBBOARDS_WON_FACTOR:          f32 = 1.0;
const SUBBOARDS_WON_PLACES_FACTOR:   f32 = 0.5;
const SUBBOARDS_PIECE_PLACES_FACTOR: f32 = 0.05;

pub fn eval(board_state: &BoardState) -> Eval {
    match board_state.state() {
        PatternState::Won(player) if player == board_state.turn() => {
            return  1000.0;
        },
        PatternState::Won(player) if player != board_state.turn() => {
            return -1000.0;
        },
        _ => (),
    }

    let mut eval = 0.0;

    eval += eval_subboards_won       (board_state) * SUBBOARDS_WON_FACTOR;
    eval += eval_subboards_won_places(board_state) * SUBBOARDS_WON_PLACES_FACTOR;
    eval += eval_piece_places        (board_state) * SUBBOARDS_PIECE_PLACES_FACTOR;

    eval
}

pub fn dbg_print_eval_breakdown(board_state: &BoardState, move_: Move, index: usize) {
    eprintln!(
        "{:>2}: eval: {:>7}, {} (sbbwon: {:>5}, sbbplc: {:>5}, pceplc: {:>5})",
        index,
        eval(board_state),
        move_.dbg_to_string(),
        eval_subboards_won       (board_state) * SUBBOARDS_WON_FACTOR,
        eval_subboards_won_places(board_state) * SUBBOARDS_WON_PLACES_FACTOR,
        eval_piece_places        (board_state) * SUBBOARDS_PIECE_PLACES_FACTOR,
    );
}
fn eval_subboards_won(board_state: &BoardState) -> Eval {
    let subboard_pattern = board_state.subboard_pattern();

    let subboards_won  = subboard_pattern.spots(board_state.turn().to_piece());
    let subboards_lost = subboard_pattern.spots(board_state.turn().opposite().to_piece());

    subboards_won.len() as Eval - subboards_lost.len() as Eval
}

fn eval_subboards_won_places(board_state: &BoardState) -> Eval {
    let subboard_pattern = board_state.subboard_pattern();

    let subboards_won  = subboard_pattern.spots(board_state.turn().to_piece());
    let subboards_lost = subboard_pattern.spots(board_state.turn().opposite().to_piece());

    places_eval(subboards_won) - places_eval(subboards_lost)
}

fn eval_piece_places(board_state: &BoardState) -> Eval {
    let own_piece_places = board_state
        .enumerate()
        .filter_map(|(place, _subboard)| board_state.pattern_if_active(place))
        .flat_map(|pattern| pattern.spots(board_state.turn().to_piece()))
        .collect();

    let opposite_piece_places = board_state
        .enumerate()
        .filter_map(|(place, _subboard)| board_state.pattern_if_active(place))
        .flat_map(|pattern| pattern.spots(board_state.turn().opposite().to_piece()))
        .collect();

    places_eval(own_piece_places) - places_eval(opposite_piece_places)
}

fn places_eval(places: Box<[Place]>) -> Eval {
    places
        .iter()
        .map(|place| {
            match place.centeredness() {
                Centeredness::Center => 1.0,
                Centeredness::Edge   => 0.5,
                Centeredness::Corner => 0.0,
            }
        })
        .reduce(|acc, place| acc + place)
        .unwrap_or(0.0)
}
