#!/bin/sh

cargo build -p mono --target x86_64-unknown-linux-musl --release
docker build -t my-game-server:1.0 .