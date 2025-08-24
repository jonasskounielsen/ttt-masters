#![crate_type = "cdylib"]

mod utils;

use std::convert::Infallible;

use utils::BoardState;

#[repr(C)]
#[derive(Debug)]
struct RawBoardState {
    board: [[RawPiece; 9]; 9],
    turn: RawTurn,
    active_subboard: RawActiveSubBoard,
}

#[repr(C)]
#[derive(Debug)]
#[allow(unused)]
enum RawActiveSubBoard {
    All      = -1,   
    TopLeft  =  0,
    TopMid   =  1,
    TopRight =  2,
    MidLeft  =  3,
    MidMid   =  4,
    MidRight =  5,
    BotLeft  =  6,
    BotMid   =  7,
    BotRight =  8,
}

#[repr(C)]
#[derive(Debug)]
struct RawMove {
    subboard:  RawPlace,
    spot:      RawPlace,
}

#[repr(C)]
#[derive(Debug)]
#[allow(unused)]
enum RawPlace {
    TopLeft  = 0,
    TopMid   = 1,
    TopRight = 2,
    MidLeft  = 3,
    MidMid   = 4,
    MidRight = 5,
    BotLeft  = 6,
    BotMid   = 7,
    BotRight = 8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
enum RawPiece {
    Cross =  1,
    Empty =  0,
    Dot   = -1,
}

#[repr(C, i32)]
#[derive(Debug)]
#[allow(unused)]
enum RawTurn {
    Cross             =  1,
    Empty(Infallible) =  0, // On the cpp side, Piece is used as Turn too
    Dot               = -1,
}

#[unsafe(no_mangle)]
extern "C" fn get_move(raw_board_state: RawBoardState) -> RawMove {
    dbg!(&raw_board_state);
    let board_state = BoardState::new(raw_board_state);
    let eligible_spots = board_state.eligible_spots();
    RawMove {
        subboard: eligible_spots[0].subboard.to_raw(),
        spot: eligible_spots[0].square.to_raw(),
    }
}