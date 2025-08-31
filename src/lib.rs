#![crate_type = "cdylib"]

use utils::{RawBoardState, RawMove};
use utils::BoardState;

mod utils;

#[unsafe(no_mangle)]
extern "C" fn get_move(raw_board_state: RawBoardState) -> RawMove {
    let board_state = BoardState::new(raw_board_state);
    let eligible_spots = board_state.eligible_spots();
    RawMove {
        subboard: eligible_spots[0].subboard.to_raw(),
        spot: eligible_spots[0].square.to_raw(),
    }
}