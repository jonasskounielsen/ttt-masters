set -e
rustc ./src/lib.rs -o ./rustbot.so
g++ ./interface/interface.cpp -fPIC -shared -o ./"AI's"/bot/libbot.so
unlink ./CROSS
unlink ./DOT
ln -sf ./"AI's"/hand ./CROSS
ln -sf ./"AI's"/bot ./DOT
steam-run ./TTT-masters