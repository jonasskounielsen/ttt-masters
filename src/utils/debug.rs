use crate::utils::{Place, Player, Subboard};

use super::{board_state::BoardState, pattern::Pattern, Piece};

impl BoardState {
    pub fn dbg_print(&self) {
        let turn = match self.turn() {
            Player::Cross => "cross",
            Player::Dot   => "dot",
        };

        eprintln!("Turn: {}", turn);
        
        let mut rows = vec![vec![String::from(" "); 9]; 9];

        for subboard_index in 0..9 {
            let subboard = self.subboard(Place::from_index(subboard_index));

            match subboard {
                Subboard::Won(player) => {
                    let character = player.dbg_character();
                    for i in 0..3 {
                        for j in 0..3 {
                            rows[(subboard_index / 3) * 3 + i][(subboard_index % 3) * 3 + j] = character.clone();
                        }
                    }
                },
                Subboard::Active  (pattern) => Self::dbg_add_pattern(pattern, &mut rows, subboard_index),
                Subboard::Inactive(pattern) => Self::dbg_add_pattern(pattern, &mut rows, subboard_index),
            }
        }
        
        for i in 0..9 {
            let row = &rows[i];
            
            if i == 3 || i == 6 {
                eprintln!("-----+-----+-----");
            }

            for j in 0..9 {
                let character = &row[j];

                eprint!("{}", character);

                if j == 2 || j == 5 {
                    eprint!("|");
                } else {
                    eprint!(" ");
                }
            }

            eprintln!("");
        }
    }

    fn dbg_add_pattern(pattern: Pattern, rows: &mut Vec<Vec<String>>, subboard_index: usize) {
        for i in 0..3 {
            for j in 0..3 {
                rows[(subboard_index / 3) * 3 + i][(subboard_index % 3) * 3 + j] = pattern.piece(Place::from_index(i * 3 + j)).dbg_character();
            }
        }
    }
}

impl Piece {
    pub fn dbg_character(&self) -> String {
        String::from(match self {
            Piece::Empty => " ",
            Piece::Cross => "X",
            Piece::Dot   => "O",
        })
    }
}

impl Player {
    pub fn dbg_character(&self) -> String {
        String::from(match self {
            Player::Cross => "X",
            Player::Dot   => "O",
        })
    }
}