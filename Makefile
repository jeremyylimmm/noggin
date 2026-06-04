CARGO_BIN := $(shell grep '^name' Cargo.toml | cut -d'"' -f2)
EXE = noggin

build:
	cargo build --release
	cp target/release/$(CARGO_BIN) $(EXE)

.PHONY: build
