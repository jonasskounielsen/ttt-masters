use crate::utils::{Place, Player, Subboard};

use super::{board_state::BoardState, pattern::Pattern, raw::{RawActiveSubBoard, RawPiece, RawTurn}, Move, Piece, RawBoardState};

impl BoardState {
    pub fn dbg_from_matrix(
        matrix: [&str; 9], active_subboard: i32, turn: &str,
    ) -> Self {
        let board: String = matrix
            .iter()
            .map(|item| *item)
            .collect::<Vec<_>>()
            .join("")
            .chars()
            .enumerate()
            .filter(|(i, _)| (i % 17) % 2 == 0)
            .map(|(_, item)| item)
            .collect();
                 
        let board = (0..9)
            .map(|i: usize| {
                (0..9)
                    .map(|j: usize| {
                        let index = (i % 3) * 3 + (i / 3) * 27 + (j / 3) * 9 + (j % 3);
                        let character = board.get(index..(index + 1)).unwrap();
                        RawPiece::dbg_from_character(&character.to_string())
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        
        let raw_board_state = RawBoardState {
            board: board,
            active_subboard: RawActiveSubBoard::dbg_from_i32(active_subboard),
            turn: RawTurn::dbg_from_character(turn),
        };

        BoardState::from_raw(raw_board_state)
    }

    pub fn dbg_print(&self) {
        let turn = match self.turn() {
            Player::Cross => "cross",
            Player::Dot   => "dot",
        };

        eprintln!("Turn: {}", turn);

        let active_subboards: Vec<_> = (0..9)
            .into_iter()
            .map(|index| {
                let subboard = self.subboard(Place::from_index(index));
                
                matches!(subboard, Subboard::Active(_))
            })
            .collect();

        for i in 0..9 {
            eprint!("{}", active_subboards[i]);

            if i % 3 == 2 {
                eprint!("\n");
            } else {
                eprint!(" ");
            }
        }
        
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
                eprintln!("------+------+------");
            }

            for j in 0..9 {
                let character = &row[j];

                eprint!("{}", character);

                if j == 2 || j == 5 {
                    eprint!(" |");
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
                rows[(subboard_index / 3) * 3 + i][(subboard_index % 3) * 3 + j] =
                    pattern.piece(Place::from_index(i * 3 + j)).dbg_character();
            }
        }
    }
}

impl Pattern {
    pub fn dbg_from_matrix(matrix: [&str; 3]) -> Self {
        let inner = matrix
            .iter()
            .map(|row| {
                row
                    .chars()
                    .step_by(2)
                    .map(|char| RawPiece::dbg_from_character(&String::from(char)))
            })
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Pattern::from_raw(inner)
    }
}

impl Piece {
    pub fn dbg_character(&self) -> String {
        String::from(match self {
            Piece::Cross => "X",
            Piece::Dot   => "O",
            Piece::Empty => " ",
        })
    }

    pub fn dbg_from_character(char: &str) -> Self {
        match char {
            "X" => Piece::Cross,
            "O" => Piece::Dot,
            " " => Piece::Empty,
            _ => panic!("invalid piece"),
        }
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

impl RawTurn {
    pub fn dbg_from_character(character: &str) -> Self {
        match character {
            "cross" => RawTurn::Cross,
            "dot" => RawTurn::Dot,
            _ => panic!("invalid turn"),
        }
    }
}

impl RawActiveSubBoard {
    pub fn dbg_from_i32(number: i32) -> Self {
        match number {
            -1 => RawActiveSubBoard::All,
             0 => RawActiveSubBoard::TopLeft,
             1 => RawActiveSubBoard::TopMid,
             2 => RawActiveSubBoard::TopRight,
             3 => RawActiveSubBoard::MidLeft,
             4 => RawActiveSubBoard::MidMid,
             5 => RawActiveSubBoard::MidRight,
             6 => RawActiveSubBoard::BotLeft,
             7 => RawActiveSubBoard::BotMid,
             8 => RawActiveSubBoard::BotRight,
             _ => panic!("invalid active subboard"),
        }
    }
}

impl RawPiece {
    pub fn dbg_from_character(character: &str) -> Self {
        match character {
            "X" => RawPiece::Cross,
            "O" => RawPiece::Dot,
            " " => RawPiece::Empty,
            _ => panic!("invalid piece"),
        }
    }
}

impl<'a> dbg_MoveList<'a> for Box<[Move<'a>]> {}

#[allow(non_camel_case_types)]
pub trait dbg_MoveList<'a>
where
    &'a Self: IntoIterator<Item = &'a Move<'a>>,
    Self: 'a,
{
    fn dbg_print(&'a self) {
        for (index, move_) in self.into_iter().enumerate() {
            let subboard = format!("{:?}", move_.subboard());
            let square   = format!("{:?}", move_.square());
            eprintln!("{:<2}: subboard: {:<8}, square: {:<8}", index, subboard, square);
        }
    }
}