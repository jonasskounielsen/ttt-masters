set -e
rustc ./src/lib.rs -o ./rustbot.so
g++ ./interface/interface.cpp -fPIC -shared -o ./"AI's"/bot/libbot.so
steam-run ./TTT-masters