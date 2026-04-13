#!/bin/bash

set -e

REPO_URL=$(git remote get-url origin)
REPO_NAME=$(basename -s .git "$REPO_URL")
CLONE_DIR="./cutechess/$REPO_NAME"

if [ -d "$CLONE_DIR/.git" ]; then
    git -C "$CLONE_DIR" fetch origin
    git -C "$CLONE_DIR" checkout main
    git -C "$CLONE_DIR" reset --hard origin/main
else
    git clone --branch main "$REPO_URL" "$CLONE_DIR"
fi

BIN_NAME=$(grep -m1 '^name' Cargo.toml | cut -d'"' -f2)

cargo build --release --manifest-path "$CLONE_DIR/Cargo.toml"

./cutechess/cutechess-cli \
    -engine name=$1 cmd=target/release/mort \
    -engine name=baseline cmd="$CLONE_DIR/target/release/$BIN_NAME" \
    -each proto=uci tc=10+0.1 \
    -games 2 \
    -rounds 5000 \
    -concurrency 4 \
    -openings file=./cutechess/book-ply8-unifen-Q-0.0-0.25.pgn format=pgn order=random \
    -sprt elo0=0.0 elo1=5.0 alpha=0.05 beta=0.05 \
