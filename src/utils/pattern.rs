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

    pub fn enumerate(&self) -> EnumeratePattern {
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
        let mut new_pattern = self.clone();
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
