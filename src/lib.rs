#![crate_type = "cdylib"]

use utils::{RawBoardState, RawMove};
use utils::BoardState;

mod utils;

mod algorithms {
    pub mod greedy;
}

#[unsafe(no_mangle)]
extern "C" fn get_move(raw_board_state: RawBoardState) -> RawMove {
    let board_state = BoardState::new(raw_board_state);
    let move_ = algorithms::greedy::greedy(board_state);
    move_.to_raw()
}