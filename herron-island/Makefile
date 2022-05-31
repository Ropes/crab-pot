
targets: pkg/herron_island_bg.wasm pkg/herron_island_bg.js pkg/herron_island.js

build: targets
	wasm-build pack

release: targets
	wasm-pack build --release --target web

clean:
	rm ./pkg/*
