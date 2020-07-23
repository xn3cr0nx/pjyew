WASM_PKG=wasm-pack
WASM_NAME=wasm
DIR=./static

.PHONY: build
# build: wasm rollup
build: 
	# reflex -r '\.rs$$' -s -- make wasm && make rollup
	reflex -r '\.rs$$' -s -- make wasm

.PHONY: wasm
wasm:
	$(WASM_PKG) build --target web --out-name $(WASM_NAME) --out-dir $(DIR)

# .PHONY: rollup
# rollup:
# 	rollup ./static/main.js --format iife --file ./static/bundle.js

