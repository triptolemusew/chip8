# Chip8

Chip8 implementation written in Rust with SDL2 for the rendering.

## Getting Started

This project support building for both the wasm binary through `wasm-pack` and natively running the emulator through sdl.

## Building

```
cargo build --features="sdl"
```

## Running

```
cargo run --features="sdl" <ROM>
```

## Development with WASM

```
cd web/
npm run start
```
