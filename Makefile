build:
	wasm-pack build --release --target web

publish:
	cd pkg;npm publish
