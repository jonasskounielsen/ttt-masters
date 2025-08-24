pub mod pattern;

use self::pattern::{Pattern, SubboardState};

use crate::{RawActiveSubBoard, RawBoardState, RawPlace, RawTurn};

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
    pub fn from_index(index: usize) -> Self {
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

pub struct Spot {
    pub subboard: Place,
    pub square:   Place,
}

pub enum Player {
    Cross,
    Dot,
}

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

pub struct BoardState {
    board: [Subboard; 9],
    turn: Player,
}

impl BoardState {
    pub fn new(raw_board_state: RawBoardState) -> Self {
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
            turn: match raw_board_state.turn {
                RawTurn::Cross => Player::Cross,
                RawTurn::Dot   => Player::Dot,
            },
        }
    }

    pub fn eligible_spots(&self) -> Box<[Spot]> {
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
            .collect()
    }
}