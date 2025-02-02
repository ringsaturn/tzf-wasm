build:
	cargo install wasm-pack
	wasm-pack build --release --target web

publish:
	cd pkg;npm publish
