pub mod pattern;

mod raw;

pub use crate::utils::raw::{RawBoardState, RawMove};

use self::pattern::{Pattern, SubboardState};

use crate::utils::raw::{RawActiveSubBoard, RawPlace, RawTurn};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Cross,
    Empty,
    Dot,
}

#[derive(Clone, Copy)]
pub enum Place {
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    MidMid,
    MidRight,
    BotLeft,
    BotMid,
    BotRight,
}

impl Place {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::TopLeft,
            1 => Self::TopMid,
            2 => Self::TopRight,
            3 => Self::MidLeft,
            4 => Self::MidMid,
            5 => Self::MidRight,
            6 => Self::BotLeft,
            7 => Self::BotMid,
            8 => Self::BotRight,
            _ => panic!("invalid place"),
        }
    }

    fn to_index(&self) -> usize {
        match *self {
            Self::TopLeft  => 0,
            Self::TopMid   => 1,
            Self::TopRight => 2,
            Self::MidLeft  => 3,
            Self::MidMid   => 4,
            Self::MidRight => 5,
            Self::BotLeft  => 6,
            Self::BotMid   => 7,
            Self::BotRight => 8,
        }
    }

    pub fn to_raw(&self) -> RawPlace {
        match *self {
            Self::TopLeft  => RawPlace::TopLeft,
            Self::TopMid   => RawPlace::TopMid,
            Self::TopRight => RawPlace::TopRight,
            Self::MidLeft  => RawPlace::MidLeft,
            Self::MidMid   => RawPlace::MidMid,
            Self::MidRight => RawPlace::MidRight,
            Self::BotLeft  => RawPlace::BotLeft,
            Self::BotMid   => RawPlace::BotMid,
            Self::BotRight => RawPlace::BotRight,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Spot {
    pub subboard: Place,
    pub square:   Place,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Cross,
    Dot,
}

impl Player {
    pub fn piece(&self) -> Piece {
        match self {
            Self::Cross => Piece::Cross,
            Self::Dot   => Piece::Dot,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Cross => Self::Dot,
            Self::Dot   => Self::Cross,
        }
    }

    pub(crate) fn from_raw(raw_turn: RawTurn) -> Self {
        match raw_turn {
            RawTurn::Cross => Player::Cross,
            RawTurn::Dot   => Player::Dot,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Subboard {
    Won(Player),
    Active  (Pattern),
    Inactive(Pattern),
}

impl Subboard {
    pub fn from_pattern(pattern: Pattern, active: bool) -> Self {
        match pattern.state() {
            SubboardState::Won(player) => Subboard::Won(player),
            SubboardState::Undecided if active => Subboard::Active  (pattern),
            SubboardState::Undecided           => Subboard::Inactive(pattern),
        }
    }
}

#[derive(Clone, Copy)]
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
    
    pub fn subboard(&mut self, subboard: Place) -> Subboard {
        self.board[subboard.to_index()]
    } 

    pub fn subboard_pattern(&self) -> Pattern {
        let pieces = self.board.map(|subboard| {
            return match subboard {
                Subboard::Won(player)
            };
        })

        Pattern::new(pieces)
    }
    
    pub fn do_move(&self, move_: Move) -> Self {
        let mut new_subboards = self.clone().board;

        let subboard = &mut new_subboards[move_.subboard().to_index()];
        
        let pattern = match subboard {
            Subboard::Won(_)      => panic!("invalid move; subboard is won"),
            Subboard::Inactive(_) => panic!("invalid move; subboard is inactive"),
            Subboard::Active(pattern) => pattern,
        };
        
        let piece: &mut Piece = pattern.piece(move_.square());

        let new_piece = self.turn.piece();
                
        match piece {
            Piece::Cross => panic!("invalid move; square is non-empty"),
            Piece::Dot   => panic!("invalid move; square is non-empty"),
            Piece::Empty => {
                *piece = new_piece;
            },
        };
        
        if let SubboardState::Won(player) = pattern.state() {
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
        self.board.iter()
            .enumerate()
            .map(|(subboard_index, subboard)| {
                match *subboard {
                    Subboard::Won(_) => Box::new([]),
                    Subboard::Inactive(_) => Box::new([]),
                    Subboard::Active(pattern) => {
                        pattern.free_spots().iter()
                            .map(|place| Spot {
                                subboard: Place::from_index(subboard_index),
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

    pub fn is_winning(&self, move_: Move) -> bool {
        let new_state = self.do_move(move_);

        let subboard_pattern = new_state.subboard_pattern();
        
        if let SubboardState::Won(player) = subboard_pattern.state() {
            return player == self.turn;
        }
        false
    }
}

#[derive(Clone, Copy)]
pub struct Move(Spot);

impl Move {
    pub fn new(spot: Spot) -> Self {
        Self(spot)
    }

    pub fn to_raw(&self) -> RawMove {
        RawMove {
            subboard: self.0.subboard.to_raw(),
            spot: self.0.square.to_raw()
        }
    }

    pub fn subboard(&self) -> Place {
        self.0.subboard
    }

    pub fn square(&self) -> Place {
        self.0.square
    }
}