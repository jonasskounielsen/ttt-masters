use std::collections::HashMap;

use crate::{algorithms::minimax::eval::{EVAL_LOST, EVAL_WON, Eval}, utils::board_state::BoardState};

#[derive(Debug, Clone, Copy, PartialEq)]
struct TranspositionEntry {
    eval: Eval,
    depth: u32,
    is_terminal: bool,
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

    pub fn get(&self, board_state: &BoardState, depth: u32) -> Option<Eval> {
        if let Some(entry) = self.table.get(board_state) &&
                   (entry.depth >= depth || entry.is_terminal) {
            return Some(entry.eval);
        }
        None
    }

    pub fn set(&mut self, board_state: &BoardState, depth: u32, eval: Eval) {
        if self.get(board_state, depth).is_some() {
            panic!("attempt to overwrite transposition table entry");
        }
        let entry = TranspositionEntry {
            eval,
            depth,
            is_terminal: eval == EVAL_WON || eval == EVAL_LOST,
        };
        self.table.insert(*board_state, entry);
    }
}
