use rustbot::{algorithms::minimax::{dbg_print_moves, minimax}, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "     .  O  .     ",
            "     .    X.     ",
            "O O O.  X X.O O O",

            "     .X X X.     ",
            "X O X.     .O   X",
            "X   X.     .O O X",
            
            "X   X.     .X   O",
            "X   O.     .O   O",
            "  O O.X X X.O    ",
        ], 3, "dot",
    );

    board_state.dbg_print();
    
    dbg_print_moves(&board_state);

    let move_ = minimax(&board_state);

    move_.dbg_print();
}
