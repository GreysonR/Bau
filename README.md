# About
A simple physics engine written in Rust as a Bevy plugin. Currently *very* early and has few features implemented. It was recently rewritten from WASM-only to a Bevy plugin intended primarily for native builds. The old version is still available under the 0.0.7-wasm branch. 

The project is intended for learning about the fundamentals of physics engines, not to be a complete physics solution for games. 

# Requirements
- [Bevy dependencies](https://bevy.org/learn/quick-start/getting-started/setup/)
- [LLD linker](https://bevy.org/learn/quick-start/getting-started/setup/#alternative-linkers)

# Running the testing GUI
`make run` or `cargo run -p bau_test_suite`
