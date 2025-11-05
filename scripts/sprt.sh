#!/bin/bash
../cutechess/cutechess-cli \
    -engine name=$1 cmd=target/release/mort \
    -engine name=baseline cmd=../baseline \
    -each proto=uci tc=10+0.1 \
    -games 2 \
    -rounds 5000 \
    -concurrency 4 \
    -openings file=../cutechess/book-ply8-unifen-Q-0.0-0.25.pgn format=pgn order=random \
    -sprt elo0=0.0 elo1=5.0 alpha=0.05 beta=0.05 \
