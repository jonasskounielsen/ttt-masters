use std::iter::Enumerate;

use crate::utils::pattern::PatternState;

use super::{pattern::Pattern, raw::RawActiveSubBoard, Move, Piece, Place, Player, RawBoardState, Spot, Subboard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardState {
    board: [Subboard; 9],
    turn: Player,
}

impl BoardState {
    pub fn from_raw(raw_board_state: RawBoardState) -> Self {
        let board = std::array::from_fn(|subboard_index| {
            let pattern = Pattern::from_raw(raw_board_state.board[subboard_index]);

            let active = match raw_board_state.active_subboard {
                RawActiveSubBoard::All      => true,
                RawActiveSubBoard::TopLeft  => subboard_index == 0,
                RawActiveSubBoard::TopMid   => subboard_index == 1,
                RawActiveSubBoard::TopRight => subboard_index == 2,
                RawActiveSubBoard::MidLeft  => subboard_index == 3,
                RawActiveSubBoard::MidMid   => subboard_index == 4,
                RawActiveSubBoard::MidRight => subboard_index == 5,
                RawActiveSubBoard::BotLeft  => subboard_index == 6,
                RawActiveSubBoard::BotMid   => subboard_index == 7,
                RawActiveSubBoard::BotRight => subboard_index == 8,
            };
            let subboard = Subboard::from_pattern(pattern, active);
            subboard
        });
        
        BoardState {
            board: board,
            turn: Player::from_raw(raw_board_state.turn),
        }
    }

    pub fn turn(&self) -> Player {
        self.turn
    }
    
    pub fn subboard(&self, subboard: Place) -> Subboard {
        self.board[subboard.to_index()]
    }

    pub fn pattern_if_active(&self, subboard: Place) -> Option<Pattern> {
        match self.subboard(subboard) {
            Subboard::Active(pattern) => Some(pattern),
            _ => None,
        }
    }

    pub fn subboard_pattern(&self) -> Pattern {
        let pieces = self.board.map(|subboard| {
            match subboard {
                Subboard::Won(player) => player.to_piece(),
                _ => Piece::Empty,
            }
        });

        Pattern::new(pieces)
    }

    pub fn enumerate(&self) -> EnumerateBoard<'_> {
        EnumerateBoard::new(
            self.board.iter()
            .enumerate()
        )
    }
    
    pub fn do_move(&self, move_: Move) -> Self {
        let mut new_subboards = self.clone().board;
        let subboard = &mut new_subboards[move_.subboard().to_index()];
        let pattern = match subboard {
            Subboard::Won(_)      => panic!("invalid move; subboard is won"),
            Subboard::Inactive(_) => panic!("invalid move; subboard is inactive"),
            Subboard::Active(pattern) => pattern,
        };
        
        let piece: &mut Piece = pattern.piece_mut(move_.square());
        let new_piece = self.turn.to_piece();
        match piece {
            Piece::Cross => panic!("invalid move; square is non-empty"),
            Piece::Dot   => panic!("invalid move; square is non-empty"),
            Piece::Empty => {
                *piece = new_piece;
            },
        };
        
        if let PatternState::Won(player) = pattern.state() {
            *subboard = Subboard::Won(player);
        }

        let new_active_subboard = &mut new_subboards[move_.square().to_index()];
        if let Subboard::Inactive(pattern) = new_active_subboard {
            *new_active_subboard = Subboard::Active(*pattern);
        }
        if let Subboard::Won(_) = new_active_subboard {
            for subboard in &mut new_subboards {
                if let Subboard::Inactive(pattern) = *subboard {
                    *subboard = Subboard::Active(pattern);
                }
            }
        }
        
        let new_turn = self.turn.opposite();
        let new_board = BoardState {
            board: new_subboards,
            turn: new_turn,
        };
        new_board
    }

    pub fn eligible_moves(&self) -> Box<[Move<'_>]> {
        self.enumerate()
            .map(|(subboard_place, subboard)| {
                match *subboard {
                    Subboard::Won(_) => Box::new([]),
                    Subboard::Inactive(_) => Box::new([]),
                    Subboard::Active(pattern) => {
                        pattern.free_spots().iter()
                            .map(|place| Spot {
                                subboard: subboard_place,
                                square: *place,
                            })
                            .collect::<Box<[Spot]>>()
                    },
                }
            })
            .flatten()
            .map(|spot| Move::new(spot))
            .collect()
    }

    pub fn state(&self) -> PatternState {
        let subboard_pattern = self.subboard_pattern();
        
        subboard_pattern.state()
    }
}

pub struct EnumerateBoard<'a> {
    iter: Enumerate<std::slice::Iter<'a, Subboard>>,
}

impl<'a> EnumerateBoard<'a> {
    fn new(iter: Enumerate<std::slice::Iter<'a, Subboard>>) -> Self {
        Self {
            iter,
        }
    }
}

impl<'a> Iterator for EnumerateBoard<'a> {
    type Item = (Place, &'a Subboard);
    
    fn next(&mut self) -> Option<Self::Item> {
        let inner_next = self.iter.next();

        inner_next.map(|inner_next|
            (Place::from_index(inner_next.0), inner_next.1)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use crate::utils::{pattern::Pattern, raw::{RawActiveSubBoard, RawPiece, RawTurn}, Move, Place, Player, RawBoardState, Spot, Subboard};

    use super::BoardState;

    const TEST_BOARD: [[&str; 3]; 9] = [
        [
            "    X",
            "  O X",
            "O    ",
        ],
        [
            "X   X",
            "  O X",
            "O    ",
        ],
        [
            "    X",
            "    X",
            "O   X",
        ],
        [
            "     ",
            "  O X",
            "O    ",
        ],
        [
            "  X X",
            "  O O",
            "O    ",
        ],
        [
            "    X",
            "  O X",
            "O X  ",
        ],
        [
            "    X",
            "O O O",
            "     ",
        ],
        [
            "O   X",
            "  O X",
            "X   O",
        ],
        [
            "    X",
            "X    ",
            "O    ",
        ],
    ];
            
    fn test_board() -> [[RawPiece; 9]; 9] {
        TEST_BOARD.map(|subboard| {
            subboard
                .iter()
                .map(|row| {
                    row
                       .chars()
                       .step_by(2)
                       .collect::<Vec<_>>()
                })
                .flatten()
                .map(|piece| RawPiece::dbg_from_character(&piece.to_string()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
    }

    #[test]
    fn from_raw_turn_subboard() {
        let winners = [
            None,
            None,
            Some(Player::Cross),
            None,
            None,
            None,
            Some(Player::Dot),
            Some(Player::Dot),
            None,
        ];

        let raw_board_state = RawBoardState {
            active_subboard: RawActiveSubBoard::MidMid,
            turn: RawTurn::Cross,
            board: test_board(),
        };
        
        let board_state = BoardState::from_raw(raw_board_state);

        assert_eq!(board_state.turn(), Player::Cross);

        for i in 0..9 {
            let subboard = board_state.subboard(Place::from_index(i));
            match subboard {
                Subboard::Active(pattern) => {
                    for j in 0..9 {
                        let piece = pattern.piece(Place::from_index(j));
                        let test_pattern = Pattern::dbg_from_matrix(TEST_BOARD[i]);
                        assert_eq!(piece, test_pattern.piece(Place::from_index(j)));
                    }
                },
                Subboard::Inactive(pattern) => {
                    for j in 0..9 {
                        let piece = pattern.piece(Place::from_index(j));
                        let test_pattern = Pattern::dbg_from_matrix(TEST_BOARD[i]);
                        assert_eq!(piece, test_pattern.piece(Place::from_index(j)));
                    }
                },
                Subboard::Won(player) => {
                    let winner = winners[i];
                    if let Some(winner) = winner {
                        assert_eq!(player, winner);
                    } else {
                        panic!();
                    }
                }
            }
        }
    }
    
    #[test]
    fn pattern_if_active() {
        let raw_board_state = RawBoardState {
            active_subboard: RawActiveSubBoard::MidMid,
            turn: RawTurn::Cross,
            board: test_board(),
        };
        
        let board_state = BoardState::from_raw(raw_board_state);
        
        let pattern = Pattern::dbg_from_matrix([
            "  X X",
            "  O O",
            "O    ",
        ]);

        assert!(matches!(board_state.pattern_if_active(Place::MidMid), Some(active_pattern) if active_pattern == pattern));
        
        for i in 0..9 {
            if i == 4 {
                continue;
            }
            let place = Place::from_index(i);
            assert!(matches!(board_state.pattern_if_active(place), None));
        }
    }

    #[test]
    fn subboard_pattern() {
        let raw_board_state = RawBoardState {
            active_subboard: RawActiveSubBoard::MidMid,
            turn: RawTurn::Cross,
            board: test_board(),
        };
        
        let board_state = BoardState::from_raw(raw_board_state);
        
        let pattern = Pattern::dbg_from_matrix([
            "    X",
            "     ",
            "O O  ",
        ]);
        
        assert_eq!(pattern, board_state.subboard_pattern());
    }

    #[test]
    fn enumerate() {
        let raw_board_state = RawBoardState {
            active_subboard: RawActiveSubBoard::All,
            turn: RawTurn::Cross,
            board: test_board(),
        };
        
        let board_state = BoardState::from_raw(raw_board_state);
        
        let manual = TEST_BOARD
            .iter()
            .enumerate()
            .map(|(index, matrix)| (
                Place::from_index(index),
                Subboard::from_pattern(Pattern::dbg_from_matrix(*matrix), true))
            )
            .collect::<Vec<_>>();
        
        assert!(board_state
            .enumerate()
            .all(|(place, subboard)| {
                let index = place.to_index();
                place == manual[index].0 &&
                *subboard == manual[index].1
            }));
    }

    #[test]
    fn do_move() {
        let raw_board_state = RawBoardState {
            active_subboard: RawActiveSubBoard::MidMid,
            turn: RawTurn::Cross,
            board: test_board(),
        };
        
        let board_state = BoardState::from_raw(raw_board_state);
        
        let move_ = Move::new(Spot {
            subboard: Place::BotLeft,
            square: Place::TopLeft,
        });
        let unwind = panic::catch_unwind(|| {
            board_state.do_move(move_)
        });
        assert!(unwind.is_err());
        
        let move_ = Move::new(Spot {
            subboard: Place::MidLeft,
            square: Place::BotMid,
        });
        let unwind = panic::catch_unwind(|| {
            board_state.do_move(move_)
        });
        assert!(unwind.is_err());
        
        let move_ = Move::new(Spot {
            subboard: Place::MidMid,
            square: Place::MidRight,
        });
        let unwind = panic::catch_unwind(|| {
            board_state.do_move(move_)
        });
        assert!(unwind.is_err());

        let move_ = Move::new(Spot {
            subboard: Place::MidMid,
            square: Place::TopLeft,
        });
        let new_board_state = board_state.do_move(move_);
        assert_eq!(new_board_state.turn(), Player::Dot);
        new_board_state
            .enumerate()
            .for_each(|(place, subboard)| {
                if place == Place::MidMid {
                    assert_eq!(new_board_state.subboard(Place::MidMid), Subboard::Won(Player::Cross));
                    return;
                }
                if place == Place::TopLeft {
                    assert!(matches!(subboard, Subboard::Active(_)));
                    return;
                }
                assert!(matches!(subboard, Subboard::Inactive(_) | Subboard::Won(_)));
            });
    }

    #[test]
    fn eligible_moves() {
        let raw_board_state = RawBoardState {
            active_subboard: RawActiveSubBoard::MidMid,
            turn: RawTurn::Cross,
            board: test_board(),
        };
        
        let board_state = BoardState::from_raw(raw_board_state);
        
        let eligible_moves = vec![
            Move::new(Spot { subboard: Place::MidMid, square: Place::TopLeft  }),
            Move::new(Spot { subboard: Place::MidMid, square: Place::MidLeft  }),
            Move::new(Spot { subboard: Place::MidMid, square: Place::BotMid   }),
            Move::new(Spot { subboard: Place::MidMid, square: Place::BotRight }),
        ];

        assert_eq!(*board_state.eligible_moves(), *eligible_moves);
    }
}