#!/usr/bin/env sh

set -e

./install_lib.sh

gcc example.c -I./include -L./target/debug/libmicroslop.so -lmicroslop -o ./bin/example
