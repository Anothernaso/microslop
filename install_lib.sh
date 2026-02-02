#!/usr/bin/env sh

set -e

cargo build

sudo cp ./include/microslop.h /usr/include/microslop.h
sudo cp ./target/debug/libmicroslop.so /usr/lib/libmicroslop.so
