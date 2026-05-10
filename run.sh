set -e
cargo build
cp ./target/debug/librustbot.so ./rustbot.so
g++ ./interface/interface.cpp -fPIC -shared -o ./AI/bot/libbot.so
steam-run ./TTT-masters
