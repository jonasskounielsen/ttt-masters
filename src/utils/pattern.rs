use std::iter::Enumerate;

use super::raw::RawPiece;
use super::Piece;
use super::Place;
use super::Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pattern([Piece; 9]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternState {
    Won(Player),
    Undecided,
}

use super::Piece::*;

impl Pattern {
    pub const WINNING_PATTERNS_CROSS: [Pattern; 8] = [
        Pattern([
            Cross, Cross, Cross,
            Empty, Empty, Empty,
            Empty, Empty, Empty,
        ]),
        Pattern([
            Empty, Empty, Empty,
            Cross, Cross, Cross,
            Empty, Empty, Empty,
        ]),
        Pattern([
            Empty, Empty, Empty,
            Empty, Empty, Empty,
            Cross, Cross, Cross,
        ]),
        Pattern([
            Cross, Empty, Empty,
            Cross, Empty, Empty,
            Cross, Empty, Empty,
        ]),
        Pattern([
            Empty, Cross, Empty,
            Empty, Cross, Empty,
            Empty, Cross, Empty,
        ]),
        Pattern([
            Empty, Empty, Cross,
            Empty, Empty, Cross,
            Empty, Empty, Cross,
        ]),
        Pattern([
            Cross, Empty, Empty,
            Empty, Cross, Empty,
            Empty, Empty, Cross,
        ]),
        Pattern([
            Empty, Empty, Cross,
            Empty, Cross, Empty,
            Cross, Empty, Empty,
        ]),
    ];
    
    pub const WINNING_PATTERNS_DOT: [Pattern; 8] = [
        Pattern([
            Dot,   Dot,   Dot,  
            Empty, Empty, Empty,
            Empty, Empty, Empty,
        ]),
        Pattern([
            Empty, Empty, Empty,
            Dot,   Dot,   Dot,  
            Empty, Empty, Empty,
        ]),
        Pattern([
            Empty, Empty, Empty,
            Empty, Empty, Empty,
            Dot,   Dot,   Dot,  
        ]),
        Pattern([
            Dot,   Empty, Empty,
            Dot,   Empty, Empty,
            Dot,   Empty, Empty,
        ]),
        Pattern([
            Empty, Dot,   Empty,
            Empty, Dot,   Empty,
            Empty, Dot,   Empty,
        ]),
        Pattern([
            Empty, Empty, Dot,  
            Empty, Empty, Dot,  
            Empty, Empty, Dot,  
        ]),
        Pattern([
            Dot,   Empty, Empty,
            Empty, Dot,   Empty,
            Empty, Empty, Dot,  
        ]),
        Pattern([
            Empty, Empty, Dot,  
            Empty, Dot,   Empty,
            Dot,   Empty, Empty,
        ]),
    ];

    pub fn new(pieces: [Piece; 9]) -> Self {
        Self(pieces)
    }
    
    pub(super) fn from_raw(raw_pattern: [RawPiece; 9]) -> Self {
        Pattern(raw_pattern.map(|raw_piece| {
            match raw_piece {
                RawPiece::Cross => Piece::Cross,
                RawPiece::Dot   => Piece::Dot,
                RawPiece::Empty => Piece::Empty,
            }
        }))
    }

    pub fn piece(&self, place: Place) -> &Piece {
        &self.0[place.to_index()]
    }
    
    pub fn piece_mut(&mut self, place: Place) -> &mut Piece {
        &mut self.0[place.to_index()]
    }

    pub fn enumerate(&self) -> EnumeratePattern<'_> {
        EnumeratePattern::new(
            self.0.iter().enumerate()
        )
    }

    pub fn contains(&self, pattern: Pattern) -> bool {
        self.enumerate()
            .all(|(place, piece)|
                match *pattern.piece(place) {
                    Cross if *piece != Cross => false,
                    Dot   if *piece != Dot   => false,
                    _ => true,
                }
            )
    }
    
    pub fn state(&self) -> PatternState {
        let cross_won = Pattern::WINNING_PATTERNS_CROSS
            .iter()
            .any(|pattern| {
                self.contains(*pattern)
            });

        if cross_won {
            return PatternState::Won(Player::Cross);
        }

        let dot_won = Pattern::WINNING_PATTERNS_DOT
            .iter()
            .any(|pattern| {
                self.contains(*pattern)
            });

        if dot_won {
            return PatternState::Won(Player::Dot);
        }

        PatternState::Undecided
    }

    pub fn free_spots(&self) -> Box<[Place]> {
        self.enumerate()
            .filter(|(_, piece)| matches!(piece, Empty))
            .map(|(place, _)| place)
            .collect()
    }

    pub fn wins(&self, square: Place, player: Player) -> bool {
        let mut new_pattern = *self;
        *new_pattern.piece_mut(square) = player.to_piece();
        matches!(new_pattern.state(), PatternState::Won(_))
    }

    pub fn blocks(&self, square: Place, player: Player) -> bool {
        self.wins(square, player.opposite())
    }
}

pub struct EnumeratePattern<'a> {
    iter: Enumerate<std::slice::Iter<'a, Piece>>,
}

impl<'a> EnumeratePattern<'a> {
    fn new(iter: Enumerate<std::slice::Iter<'a, Piece>>) -> Self {
        Self {
            iter,
        }
    }
}

impl<'a> Iterator for EnumeratePattern<'a> {
    type Item = (Place, &'a Piece);
    
    fn next(&mut self) -> Option<Self::Item> {
        let inner_next = self.iter.next();

        inner_next.map(|inner_next|
            (Place::from_index(inner_next.0), inner_next.1)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{pattern::PatternState, raw::RawPiece, Piece, Place, Player};

    use super::Pattern;

    #[test]
    fn from_raw_piece_piece_mut() {
        let raw_pattern = [
            RawPiece::Cross, RawPiece::Dot,   RawPiece::Empty,
            RawPiece::Empty, RawPiece::Empty, RawPiece::Empty,
            RawPiece::Empty, RawPiece::Empty, RawPiece::Empty,
        ];
        let mut pattern = Pattern::from_raw(raw_pattern);
        assert_eq!(*pattern.piece    (Place::TopLeft),  Piece::Cross);
        assert_eq!(*pattern.piece_mut(Place::TopMid),   Piece::Dot);
        assert_eq!(*pattern.piece    (Place::TopRight), Piece::Empty);
        assert_eq!(*pattern.piece_mut(Place::MidMid),   Piece::Empty);
    }

    #[test]
    fn enumerate() {
        let pattern = Pattern::dbg_from_matrix([
            "X    ",
            "  X O",
            "X O O",
        ]);
        let pattern_data = [
            Piece::Cross, Piece::Empty, Piece::Empty,
            Piece::Empty, Piece::Cross, Piece::Dot,
            Piece::Cross, Piece::Dot,   Piece::Dot,
        ];
        let enumerated = pattern_data
            .iter()
            .enumerate()
            .map(|(index, piece)| (Place::from_index(index), piece))
            .collect::<Vec<_>>();
        assert_eq!(pattern.enumerate().collect::<Vec<_>>(), enumerated);
    }

    #[test]
    fn contains() {
        let pattern = Pattern::dbg_from_matrix([
            "X    ",
            "  X O",
            "X O O",
        ]);
        let contained = Pattern::dbg_from_matrix([
            "X    ",
            "  X O",
            "X O O",
        ]);
        assert!(pattern.contains(contained));
        let also_contained = Pattern::dbg_from_matrix([
            "X    ",
            "     ",
            "  O O",
        ]);
        assert!(pattern.contains(also_contained));
        let also_also_contained = Pattern::dbg_from_matrix([
            "     ",
            "     ",
            "     ",
        ]);
        assert!(pattern.contains(also_also_contained));
        let not_contained = Pattern::dbg_from_matrix([
            "X   O",
            "  X  ",
            "  O  ",
        ]);
        assert!(!pattern.contains(not_contained));
    }

    #[test]
    fn state() {
        let won_cross = Pattern::dbg_from_matrix([
            "X X X",
            "  O  ",
            "O   O",
        ]);
        assert_eq!(won_cross.state(), PatternState::Won(Player::Cross));
        let won_dot = Pattern::dbg_from_matrix([
            "X    ",
            "O O O",
            "X   X",
        ]);
        assert_eq!(won_dot.state(),   PatternState::Won(Player::Dot));
        let undecided = Pattern::dbg_from_matrix([
            "X O X",
            "X O O",
            "O X O",
        ]);
        assert_eq!(undecided.state(), PatternState::Undecided);
    }

    #[test]
    fn free_spots() {
        let pattern = Pattern::dbg_from_matrix([
            "X O  ",
            "  X O",
            "O   X",
        ]);
        let free_spots = [
            Place::TopRight, Place::MidLeft, Place::BotMid,
        ];
        pattern
            .free_spots()
            .iter()
            .enumerate()
            .for_each(|(index, place)| {
                assert_eq!(free_spots[index], *place);
            });
    }

    #[test]
    fn wins_blocks() {
        let winnable_cross = Pattern::dbg_from_matrix([
            "X   X",
            "  O  ",
            "O    ",
        ]);
        assert!(winnable_cross.wins  (Place::TopMid, Player::Cross));
        assert!(winnable_cross.blocks(Place::TopMid, Player::Dot));
        let winnable_dot = Pattern::dbg_from_matrix([
            "  X  ",
            "O   O",
            "X O X",
        ]);
        assert!(winnable_dot.wins  (Place::MidMid, Player::Dot));
        assert!(winnable_dot.blocks(Place::MidMid, Player::Cross));
        let winnable_both = Pattern::dbg_from_matrix([
            "O X O",
            "O   O",
            "  X X",
        ]);
        assert!(winnable_both.wins  (Place::BotLeft, Player::Dot));
        assert!(winnable_both.blocks(Place::BotLeft, Player::Cross));
        assert!(winnable_both.wins  (Place::BotLeft, Player::Cross));
        assert!(winnable_both.blocks(Place::BotLeft, Player::Dot));
    }
}