# RV32i CPU implemented in [Turing Complete](https://turingcomplete.game/)

This repository contains what's needed to compile rust code for a RISC-V CPU I
made in Turing Complete.

![Schematic Collage](/screenshots/collage.jpg?raw=true)

Base off:
* https://github.com/defermelowie/bare-metal-rust-on-riscv
* https://github.com/rust-embedded/riscv-rt

## Usage

Simply compile with `cargo build --release`. You can then import the built
executable `target/riscv32i-unknown-none-elf/release/turning-complete-riscv`
into Turing Complete and press run!

Currently the only IO implemented is for the 80x24 console.

## Save Game

I've included the save game under `save/circuit.data`. The circuit was made
under version 0.1055 Beta.