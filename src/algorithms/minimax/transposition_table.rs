use std::collections::HashMap;

use crate::{algorithms::minimax::eval::{EVAL_LOST, EVAL_WON, Eval}, utils::{Move, board_state::BoardState}};

#[derive(Debug, Clone, Copy, PartialEq)]
struct TranspositionEntry {
    eval: Eval,
    depth: u32,
    is_terminal: bool,
    best_move: Option<Move>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TranspositionTableResponse {
    NotPresent,
    PresentHighDepth {
        eval: Eval,
        best_move: Option<Move>,
    },
    PresentLowDepth {
        eval: Eval,
        best_move: Option<Move>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TranspositionTable {
    table: HashMap<BoardState, TranspositionEntry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, board_state: &BoardState, depth: u32) -> TranspositionTableResponse {
        if let Some(entry) = self.table.get(board_state) {
            if entry.depth >= depth || entry.is_terminal {
                TranspositionTableResponse::PresentHighDepth {
                    eval: entry.eval,
                    best_move: entry.best_move,
                }
            } else {
                TranspositionTableResponse::PresentLowDepth {
                    eval: entry.eval,
                    best_move: entry.best_move,
                }
            }
        } else {
            TranspositionTableResponse::NotPresent
        }
    }

    pub fn set(&mut self, board_state: &BoardState, depth: u32, eval: Eval, best_move: Option<Move>) {
        if let Some(TranspositionEntry { depth: entry_depth, .. }) = self.table.get(board_state) &&
            *entry_depth >= depth {
            return;
        }
        let entry = TranspositionEntry {
            eval,
            depth,
            is_terminal: eval == EVAL_WON || eval == EVAL_LOST,
            best_move,
        };
        self.table.insert(*board_state, entry);
    }
}
