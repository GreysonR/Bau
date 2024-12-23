# Install dependencies
install:
	npm i
	npm i -g nodemon
	npm i -g concurrently
	cargo install watchexec-cli
	npm run build-wasm

# Start hot reload for app
run:
	concurrently --kill-others "nodemon" "npm run watch-wasm"

# Build final bundle
build:
	npm run build