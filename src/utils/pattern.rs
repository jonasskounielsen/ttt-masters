use crate::RawPiece;

use super::Piece;
use super::Place;
use super::Player;

#[derive(Clone, Copy)]
pub struct Pattern([Piece; 9]);

pub enum SubboardState {
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
    
    pub fn from_raw(raw_subboard: [RawPiece; 9]) -> Self {
        Pattern(raw_subboard.map(|raw_piece| {
            match raw_piece {
                RawPiece::Cross => Piece::Cross,
                RawPiece::Dot   => Piece::Dot,
                RawPiece::Empty => Piece::Empty,
            }
        }))
    }

    pub fn contains(&self, pattern: Pattern) -> bool {
        for i in 0..8 {
            let piece = self.0[i];
            match pattern.0[i] {
                Cross if piece != Cross => return false,
                Dot   if piece != Dot   => return false,
                _ => (),
            }
        }
        true
    }
    
    pub fn state(&self) -> SubboardState {
        if Pattern::WINNING_PATTERNS_CROSS.iter().any(|pattern| {
            self.contains(*pattern)
        }) {
            return SubboardState::Won(Player::Cross);
        }

        if Pattern::WINNING_PATTERNS_DOT.iter().any(|pattern| {
            self.contains(*pattern)
        }) {
            return SubboardState::Won(Player::Dot);
        }

        return SubboardState::Undecided;
    }

    pub fn free_spots(&self) -> Box<[Place]> {
        self.0.iter()
            .enumerate()
            .filter(|(_, piece)| matches!(piece, Empty))
            .map(|(index, _)| {
                Place::from_index(index)
            })
            .collect()
    }
}