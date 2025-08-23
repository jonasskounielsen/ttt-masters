#![crate_type = "cdylib"]

use std::{convert::Infallible};

#[repr(C)]
#[derive(Debug)]
struct RawBoardState {
    board: [[Piece; 9]; 9],
    turn: Turn,
    sub_board: ActiveSubBoard,
}

#[repr(C)]
#[derive(Debug)]
#[allow(unused)]
enum ActiveSubBoard {
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
struct Move {
    sub:  Place,
    spot: Place,
}

#[repr(C)]
#[derive(Debug)]
#[allow(unused)]
enum Place {
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
#[derive(Debug)]
#[allow(unused)]
enum Piece {
    Cross =  1,
    Empty =  0,
    Dot   = -1,
}

#[repr(C, i32)]
#[derive(Debug)]
#[allow(unused)]
enum Turn {
    Cross             =  1,
    Empty(Infallible) =  0, // On the cpp side, Piece is used as Turn too
    Dot               = -1,
}

#[unsafe(no_mangle)]
extern "C" fn get_move(raw_board_state: RawBoardState) -> Move {
    dbg!(raw_board_state);
    Move {
        sub: Place::MidMid,
        spot: Place::MidMid,
    }
}