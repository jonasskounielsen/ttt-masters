#![crate_type = "cdylib"]

#[repr(C)]
struct Move {
    sub:  Place,
    spot: Place,
}

#[repr(C)]
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
enum Piece {
    Cross =  1,
    Empty =  0,
    Dot   = -1,
}

#[unsafe(no_mangle)]
extern "C" fn bot() -> Move {
    Move {
        sub: Place::MidMid,
        spot: Place::MidMid,
    }
}
