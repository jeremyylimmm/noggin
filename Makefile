EXE = noggin

build:
	cargo build --release
	cp target/release/$(EXE) $(EXE)

.PHONY: build
