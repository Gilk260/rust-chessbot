#!/bin/sh

cargo build --release

echo "uci
position fen rnbqkbnr/pppp1ppp/4p3/3Q4/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1
go movetime 1000" | ./target/release/chessengine
