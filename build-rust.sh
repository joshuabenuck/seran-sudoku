#!/bin/sh -e

cargo build --release
cp target/release/sudoku-rust-port sudoku-rust-port.html
