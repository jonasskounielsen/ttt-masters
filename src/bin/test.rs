use rustbot::{algorithms::greedy::greedy, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "     .     .     ",
            "  O  .     .     ",
            "     .     .  X X",

            "     .X X X.    X",
            "  O  .X X X.     ",
            "     .X X X.     ",
            
            "     .     .     ",
            "  O  .  O  .  O  ",
            "     .     .     ",
        ], 2, "dot",
    );

    board_state.dbg_print();
    
    let move_ = greedy(&board_state);

    eprintln!("{:?}", move_);
}