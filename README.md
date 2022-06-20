# Chip8

Chip8 implementation written in Rust with SDL2 for the rendering.

## Getting Started

This project support building for both the wasm binary through `wasm-pack` and natively running the emulator through sdl.


Running natively:

```
cargo run --features="sdl" <ROM>
```
