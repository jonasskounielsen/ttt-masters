use rustbot::{algorithms::{minimax::minimax}, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "     .     .X X X",
            "     .     .X X X",
            "     .     .X X X",

            "     .X X X.     ",
            "     .X X X.     ",
            "     .X X X.     ",
            
            "     .     .     ",
            "  X X.     .     ",
            "     .     .     ",
        ], 6, "dot",
    );

    board_state.dbg_print();
    
    let move_ = minimax(&board_state);

    move_.dbg_print();
}
