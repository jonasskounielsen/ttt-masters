use rustbot::{algorithms::greedy::greedy, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "                 ",
            "                 ",
            "    X            ",
            "      O          ",
            "                 ",
            "                 ",
            "                 ",
            "                 ",
            "                 ",
        ], -1, "dot",
    );

    board_state.dbg_print();
    
    let move_ = greedy(&board_state);

    eprintln!("{:?}", move_);
}