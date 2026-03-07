use rustbot::{algorithms::minimax::{dbg_print_moves, minimax}, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "     .     .     ",
            "  O  .     .     ",
            "     .     .     ",

            "     .     .     ",
            "     .  X  .     ",
            "     .X O O.     ",
            
            "     .     .X    ",
            "  O  .  X  .     ",
            "     .     .     ",
        ], 6, "dot",
    );

    board_state.dbg_print();
    
    dbg_print_moves(&board_state);

    let move_ = minimax(&board_state);

    move_.dbg_print();
}
