use rustbot::{algorithms::greedy::greedy, utils::board_state::BoardState};

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            "X X X X X X X X X",
            "X X X X X X X X X",
            "X X X X X X X X X",
            "O O X O O O      ",
            "      O O O      ",
            "      O O O      ",
            "O     O O O      ",
            "  X   O O O      ",
            "  X   O O O      ",
        ], -1, "dot",
    );
    
    let move_ = greedy(board_state);

    eprintln!("{:?}", move_);
}