build:
	cargo install wasm-pack
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --release --target web

publish:
	cd pkg;npm publish
