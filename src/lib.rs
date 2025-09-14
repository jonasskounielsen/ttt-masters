use utils::{board_state::BoardState, RawBoardState, RawMove};

pub mod utils;

pub mod algorithms {
    pub mod greedy;
}

#[unsafe(no_mangle)]
extern "C" fn get_move(raw_board_state: RawBoardState) -> RawMove {
    let board_state = BoardState::from_raw(raw_board_state);
    board_state.dbg_print();
    let move_ = algorithms::greedy::greedy(&board_state);
    move_.to_raw()
}