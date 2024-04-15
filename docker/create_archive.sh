#!/bin/sh

mkdir archive

# binary
cp target/release/mono archive/mono
# config
cp -r config archive/config
# rbin
cp -r game_data/rbin archive/rbin

zip -r mono.zip archive
rm -rf archive