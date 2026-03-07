use crate::utils::{Move, board_state::BoardState, pattern::PatternState};

pub type Eval = f32;

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

    eval += eval_terms::eval_game_almost_won       (board_state);
    eval += eval_terms::eval_subboards_won         (board_state);
    eval += eval_terms::eval_subboards_won_places  (board_state);
    eval += eval_terms::subboards_almost_won       (board_state);
    eval += eval_terms::subboards_doubly_almost_won(board_state);
    eval += eval_terms::eval_piece_places          (board_state);
    eval += eval_terms::eval_active_subboard_pieces(board_state);

    eval
}

pub fn dbg_print_eval_breakdown(board_state: &BoardState, move_: Move, index: usize) {
    eprintln!(
        "{:>2}: eval: {:>7}, {} (gmealm: {:>5}, sbbwon: {:>5}, sbbplc: {:>5},\n\
        >                  sbbalm: {:>5}, sbbdal: {:>5}, pceplc: {:>5}, apcplc: {:>5})",
        index,
        eval(board_state),
        move_.dbg_to_string(),
        eval_terms::eval_game_almost_won       (board_state),
        eval_terms::eval_subboards_won         (board_state),
        eval_terms::eval_subboards_won_places  (board_state),
        eval_terms::subboards_almost_won       (board_state),
        eval_terms::subboards_doubly_almost_won(board_state),
        eval_terms::eval_piece_places          (board_state),
        eval_terms::eval_active_subboard_pieces(board_state),
    );
}

mod eval_terms {
    use crate::{algorithms::minimax::eval::Eval, utils::{Centeredness, Place, board_state::BoardState}};

    const GAME_ALMOST_WON_FACTOR:        f32 = 1.0;
    const SUBBOARDS_WON_FACTOR:          f32 = 1.0;
    const SUBBOARDS_WON_PLACES_FACTOR:   f32 = 0.5;
    const SUBBOARDS_ALMOST_WON:          f32 = 0.3;
    const SUBBOARDS_DOUBLY_ALMOST_WON:   f32 = 0.15;
    const PIECE_PLACES_FACTOR:           f32 = 0.05;
    const ACTIVE_SUBBOARD_PIECES_FACTOR: f32 = 0.1;

    pub fn eval_game_almost_won(board_state: &BoardState) -> Eval {
        let subboard_pattern = board_state.subboard_pattern();
        let mut count = 0.0;

        if subboard_pattern.almost_won_by(board_state.turn()) {
            count += 1.0;
        }
        if subboard_pattern.doubly_almost_won_by(board_state.turn()) {
            count += 1.0;
        }
        if subboard_pattern.almost_won_by(board_state.turn().opposite()) {
            count -= 1.0;
        }
        if subboard_pattern.doubly_almost_won_by(board_state.turn().opposite()) {
            count -= 1.0;
        }
        count * GAME_ALMOST_WON_FACTOR
    }

    pub fn eval_subboards_won(board_state: &BoardState) -> Eval {
        let subboard_pattern = board_state.subboard_pattern();

        let subboards_won  = subboard_pattern.spots(board_state.turn().to_piece());
        let subboards_lost = subboard_pattern.spots(board_state.turn().opposite().to_piece());

        (subboards_won.len() as Eval - subboards_lost.len() as Eval) * SUBBOARDS_WON_FACTOR
    }

    pub fn subboards_almost_won(board_state: &BoardState) -> Eval {
        let mut count = 0.0;

        count += board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_undecided()
                    .map(|pattern| pattern.almost_won_by(board_state.turn()))
                    .unwrap_or(false)
            })
            .count() as Eval;

        count -= board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_undecided()
                    .map(|pattern| pattern.almost_won_by(board_state.turn().opposite()))
                    .unwrap_or(false)
            })
            .count() as Eval;

        count * SUBBOARDS_ALMOST_WON
    }

    pub fn subboards_doubly_almost_won(board_state: &BoardState) -> Eval {
        let mut count = 0.0;

        count += board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_undecided()
                    .map(|pattern| pattern.doubly_almost_won_by(board_state.turn()))
                    .unwrap_or(false)
            })
            .count() as Eval;

        count -= board_state
            .enumerate()
            .filter(|(_, subboard)| {
                subboard
                    .pattern_if_undecided()
                    .map(|pattern| pattern.doubly_almost_won_by(board_state.turn().opposite()))
                    .unwrap_or(false)
            })
            .count() as Eval;

        count * SUBBOARDS_DOUBLY_ALMOST_WON
    }

    pub fn eval_subboards_won_places(board_state: &BoardState) -> Eval {
        let subboard_pattern = board_state.subboard_pattern();

        let subboards_won  = subboard_pattern.spots(board_state.turn().to_piece());
        let subboards_lost = subboard_pattern.spots(board_state.turn().opposite().to_piece());

        places_eval(subboards_won) - places_eval(subboards_lost) * SUBBOARDS_WON_PLACES_FACTOR
    }

    pub fn eval_piece_places(board_state: &BoardState) -> Eval {
        let own_piece_places = board_state
            .enumerate()
            .filter_map(|(place, _subboard)| board_state.pattern_if_undecided(place))
            .flat_map(|pattern| pattern.spots(board_state.turn().to_piece()))
            .collect();

        let opposite_piece_places = board_state
            .enumerate()
            .filter_map(|(place, _subboard)| board_state.pattern_if_undecided(place))
            .flat_map(|pattern| pattern.spots(board_state.turn().opposite().to_piece()))
            .collect();

        places_eval(own_piece_places) - places_eval(opposite_piece_places) * PIECE_PLACES_FACTOR
    }

    pub fn eval_active_subboard_pieces(board_state: &BoardState) -> Eval {
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

        places_eval(own_piece_places) - places_eval(opposite_piece_places) * ACTIVE_SUBBOARD_PIECES_FACTOR
    }

    fn places_eval(places: Box<[Place]>) -> Eval {
        places
            .iter()
            .map(|place| {
                match place.centeredness() {
                    Centeredness::Center => 1.0,
                    Centeredness::Corner => 0.75,
                    Centeredness::Edge   => 0.5,
                }
            })
            .reduce(|acc, place| acc + place)
            .unwrap_or(0.0)
    }
}
