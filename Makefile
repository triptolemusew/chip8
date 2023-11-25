run:
	cargo run --features="sdl" --bin sdl

build-native:
	cargo build --features="sdl" --bin sdl

build-web:
	wasm-pack build --features="wasm"
