#include <dlfcn.h>
#include <iostream>
#include <vector>

enum piece {
    CROSS = 1,
    EMPTY = 0,
    DOT = -1
};

struct _move { // avoid name clash with move keyword
    int sub; 
    int spot;
};

struct raw_boardstate {
    std::vector<std::vector<piece>> board = std::vector<std::vector<piece>>(9, std::vector<piece>(9, piece::EMPTY));
    piece turn; 
    short current; 
};

extern "C" {
    struct ffi_safe_boardstate {
        piece board[9][9];
        piece turn;
        int current;
    };

    _move bot(raw_boardstate boardstate) {
        ffi_safe_boardstate ffi_safe_state = {};
        
        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                ffi_safe_state.board[i][j] = boardstate.board[i][j];
            }
        }
        
        ffi_safe_state.turn = boardstate.turn;

        ffi_safe_state.current = (int) boardstate.current;

        void* handle = dlopen("./rustbot.so", RTLD_LAZY);

        dlerror();

        typedef _move (*bot_move_fn)(ffi_safe_boardstate);
        bot_move_fn get_move = (bot_move_fn) dlsym(handle, "get_move");

        _move move = get_move(ffi_safe_state);

        dlclose(handle);
        
        return move;
    };
}