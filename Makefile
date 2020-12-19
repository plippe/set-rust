wasm-build:
	wasm-pack build --target web --out-dir static/pkg

wasm-run:
	python3 -m http.server --directory static

wasm-watch:
	cargo watch \
		--watch src \
		--shell 'make wasm-build' \
		--shell 'make wasm-run'
