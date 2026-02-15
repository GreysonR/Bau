.PHONY: build run

build:
	cargo build -p bau_test_suite

run:
	cargo run -p bau_test_suite