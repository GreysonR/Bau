# Installation
`make install` or install everything below manually
If using make, ensure you already have cargo and npm installed.

## Required
 * [cargo](https://rustup.rs/)
 * node and npm (if installing manually, don't forget to `npm install`)
 * nodemon: `npm i -g nodemon`
 * wasm-pack: `cargo install wasm-pack`
 * watchexec: `cargo install watchexec`

On first run, make sure to build cargo before running webpack using `npm run build-wasm` or webpack will fail

## Optional
 * concurrently: `npm i -g concurrently`

Add this to VSCode workspace settings.json to set up rust-analyzer:
```JSON
"rust-analyzer.linkedProjects": [
	"src/rust/Cargo.toml"
]
```

# Running
`make run` or `npm start`<br>
If that doesn't work, or you want their outputs separated, run in separate terminals, both from project root: `nodemon` and `npm run watch-wasm`.<br>
DO NOT run `nodemon .` (note the ".") as it will not work!<br>
The app will then be available on localhost:80

# Building
`make build` or `npm run build`
