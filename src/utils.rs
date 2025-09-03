pub mod pattern;

pub mod board_state;

pub mod debug;

mod raw;

pub use crate::utils::raw::{RawBoardState, RawMove};

use self::pattern::{Pattern, PatternState};

use crate::utils::raw::{RawPlace, RawTurn};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Cross,
    Dot,
}

impl Player {
    pub fn to_piece(&self) -> Piece {
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
            PatternState::Won(player) => Subboard::Won(player),
            PatternState::Undecided if active => Subboard::Active  (pattern),
            PatternState::Undecided           => Subboard::Inactive(pattern),
        }
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