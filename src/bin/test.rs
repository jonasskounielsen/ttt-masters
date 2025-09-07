use rustbot::utils::board_state::BoardState;

fn main() {
    let board_state = BoardState::dbg_from_matrix(
        [
            ["x", "x", "x", "x", "x", "x", "x", "x", "x"],
            ["x", "x", "x", "x", "x", "x", "x", "x", "x"],
            ["x", "x", "x", "x", "x", "x", "x", "x", "x"],
            ["o", "o", "x", "o", "o", "o", " ", " ", " "],
            [" ", " ", " ", "o", "o", "o", " ", " ", " "],
            [" ", " ", " ", "o", "o", "o", " ", " ", " "],
            ["o", " ", " ", "o", "o", "o", " ", " ", " "],
            [" ", "x", " ", "o", "o", "o", " ", " ", " "],
            [" ", "x", " ", "o", "o", "o", " ", " ", " "],
        ], -1, "dot",
    );
}