#!/bin/bash

# Install Rust and target tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add thumbv7m-none-eabi   # For STM32F103
rustup target add thumbv7em-none-eabihf # For STM32F407
cargo install cargo-binutils
rustup component add llvm-tools-preview

# Install ARM GCC Toolchain
sudo apt update
sudo apt install -y gcc-arm-none-eabi libnewlib-arm-none-eabi
