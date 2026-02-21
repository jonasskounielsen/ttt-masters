use std::convert::Infallible;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawBoardState {
    pub board: [[RawPiece; 9]; 9],
    pub turn: RawTurn,
    pub active_subboard: RawActiveSubBoard,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawMove {
    pub subboard: RawPlace,
    pub square:   RawPlace,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum RawActiveSubBoard {
    All    = -1,   
    TopLef =  0,
    TopMid =  1,
    TopRig =  2,
    MidLef =  3,
    MidMid =  4,
    MidRig =  5,
    BotLef =  6,
    BotMid =  7,
    BotRig =  8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum RawPlace {
    TopLef = 0,
    TopMid = 1,
    TopRig = 2,
    MidLef = 3,
    MidMid = 4,
    MidRig = 5,
    BotLef = 6,
    BotMid = 7,
    BotRig = 8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum RawPiece {
    Cross =  1,
    Empty =  0,
    Dot   = -1,
}

#[repr(C, i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum RawTurn {
    Cross             =  1,
    Empty(Infallible) =  0, // On the cpp side, Piece is used as Turn too
    Dot               = -1,
}
