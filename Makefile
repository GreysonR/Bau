.PHONY: *

build:
	distrobox enter bevy-dev -- cargo build --release

run:
	distrobox enter bevy-dev -- cargo build
	cargo run

clean:
	cargo clean
