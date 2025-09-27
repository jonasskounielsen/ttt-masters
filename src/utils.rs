pub mod pattern;

pub mod board_state;

pub mod debug;

mod raw;

use std::marker::PhantomData;

use board_state::BoardState;

pub use crate::utils::raw::{RawBoardState, RawMove};

use self::pattern::{Pattern, PatternState};

use crate::utils::raw::{RawPlace, RawTurn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Cross,
    Empty,
    Dot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn centeredness(&self) -> Centeredness {
        match self {
            Place::TopLeft  => Centeredness::Corner,
            Place::TopMid   => Centeredness::Edge,
            Place::TopRight => Centeredness::Corner,
            Place::MidLeft  => Centeredness::Edge,
            Place::MidMid   => Centeredness::Center,
            Place::MidRight => Centeredness::Edge,
            Place::BotLeft  => Centeredness::Corner,
            Place::BotMid   => Centeredness::Edge,
            Place::BotRight => Centeredness::Corner,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Centeredness {
    Center,
    Edge,
    Corner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move<'a> {
    spot: Spot,
    context: PhantomData<&'a BoardState>,
}

impl<'a> Move<'a> {
    pub fn new(spot: Spot) -> Self {
        Self {
            spot,
            context: PhantomData,
        }
    }

    pub fn to_raw(&self) -> RawMove {
        RawMove {
            subboard: self.spot.subboard.to_raw(),
            square:     self.spot.square.to_raw()
        }
    }

    pub fn subboard(&self) -> Place {
        self.spot.subboard
    }

    pub fn square(&self) -> Place {
        self.spot.square
    }
}

#[cfg(test)]
mod tests {
    mod place {
        use std::panic;

        use crate::utils::{Centeredness, Place};

        #[test]
        fn to_from_index() {
            for i in 0..9 {
                assert_eq!(i, Place::from_index(i).to_index());
            }
            let unwind = panic::catch_unwind(|| {
                Place::from_index(9)
            });
            assert!(unwind.is_err());
        }

        #[test]
        fn to_raw() {
            for i in 0..9 {
                let place = Place::from_index(i);
                assert_eq!(i as i32, place.to_raw() as i32);
            }
        }

        #[test]
        fn centeredness() {
            for i in 0..9 {
                let centeredness = match i {
                    4 => Centeredness::Center,
                    _ if i % 2 == 0 => Centeredness::Corner,
                    _ => Centeredness::Edge,
                };
                assert_eq!(centeredness, Place::from_index(i).centeredness());
            }
        }
    }

    mod player {
        use crate::utils::{raw::RawTurn, Piece, Player};

        #[test]
        fn to_piece_opposite_from_raw() {
            let player = Player::Cross;
            assert_eq!(player.to_piece(), Piece::Cross);
            assert_eq!(player, Player::from_raw(RawTurn::Cross));
            let opposite = player.opposite();
            assert_eq!(opposite.to_piece(), Piece::Dot);
            assert_eq!(player, Player::from_raw(RawTurn::Dot));
        }
    }

    mod subboard {
        use crate::utils::{pattern::Pattern, Player, Subboard};

        #[test]
        fn from_pattern() {
            let won_pattern = Pattern::dbg_from_matrix([
                "X X X",
                "  O  ",
                "O   O",
            ]);
            let won_subboard = Subboard::from_pattern(won_pattern, true);
            assert_eq!(Subboard::Won(Player::Cross), won_subboard);
            let undecided_pattern = Pattern::dbg_from_matrix([
                "X O  ",
                "O X  ",
                "    O",
            ]);
            let active_subboard = Subboard::from_pattern(undecided_pattern, true);
            assert!(matches!(active_subboard, Subboard::Active(_)));
            let inactive_subboard = Subboard::from_pattern(undecided_pattern, false);
            assert!(matches!(inactive_subboard, Subboard::Inactive(_)));
        }
    }

    mod move_ {
        use crate::utils::{raw::RawPlace, Move, Place, Spot};

        #[test]
        fn subboard_square_to_raw() {
            let move_ = Move::new(Spot {
                subboard: Place::TopLeft,
                square:   Place::BotRight,
            });
            assert_eq!(move_.subboard(), Place::TopLeft);
            assert_eq!(move_.square(),   Place::BotRight);
            let raw_move = move_.to_raw();
            assert_eq!(raw_move.subboard, RawPlace::TopLeft);
            assert_eq!(raw_move.square,   RawPlace::BotRight);
        }
    }
}