default: build

all: test

test: build
		cargo test

build:
		cargo build --target wasm32-unknown-unknown --release
		find target/wasm32-unknown-unknown/release/ -type f -maxdepth 1 -name "*.wasm" -exec mv {} dist/ \;

build-opt:
		cargo build --target wasm32-unknown-unknown --release
		find target/wasm32-unknown-unknown/release -type f -maxdepth 1 -name "*.wasm" -exec soroban contract optimize --wasm {} \;
		find target/wasm32-unknown-unknown/release/ -type f -maxdepth 1 -name "*.optimized.wasm" -exec mv {} dist/ \;

fmt:
		cargo fmt --all

clean:
		cargo clean