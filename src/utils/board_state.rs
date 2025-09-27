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
            return match subboard {
                Subboard::Won(player) => player.to_piece(),
                _ => Piece::Empty,
            };
        });

        Pattern::new(pieces)
    }

    pub fn enumerate(&self) -> EnumerateBoard {
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
            *subboard =  Subboard::Won(player);
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

    pub fn eligible_moves(&self) -> Box<[Move]> {
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