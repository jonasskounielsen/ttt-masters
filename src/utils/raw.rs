use std::convert::Infallible;

#[repr(C)]
#[derive(Debug)]
pub struct RawBoardState {
    pub board: [[RawPiece; 9]; 9],
    pub turn: RawTurn,
    pub active_subboard: RawActiveSubBoard,
}

#[repr(C)]
#[derive(Debug)]
pub struct RawMove {
    pub subboard:  RawPlace,
    pub spot:      RawPlace,
}

#[repr(C)]
#[derive(Debug)]
#[allow(unused)]
pub enum RawActiveSubBoard {
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
#[allow(unused)]
pub enum RawPlace {
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
pub enum RawPiece {
    Cross =  1,
    Empty =  0,
    Dot   = -1,
}

#[repr(C, i32)]
#[derive(Debug)]
#[allow(unused)]
pub enum RawTurn {
    Cross             =  1,
    Empty(Infallible) =  0, // On the cpp side, Piece is used as Turn too
    Dot               = -1,
}