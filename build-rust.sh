#!/bin/sh -e

cargo build --release --bin sudoku-rust-port
cp target/release/sudoku-rust-port sudoku-rust-port.html
cargo build --release --bin sudoku-rust-idioms
cp target/release/sudoku-rust-idioms sudoku-rust-idioms.html
