use rustbot::{algorithms::{greedy::greedy, minimax::minimax}, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "O O O.X X X.X X X",
            "O O O.X X X.X X X",
            "O O O.X X X.X X X",

            "X X X.O O O.O O O",
            "X X X.O O O.O O O",
            "X X X.O O O.O O O",
            
            "X X X..X X X",
            "X X X.O O X.X X X",
            "X X X.  O X.X X X",
        ], -1, "cross",
    );

    board_state.dbg_print();
    
    let move_ = minimax(&board_state);

    eprintln!("{:?}", move_);
}
