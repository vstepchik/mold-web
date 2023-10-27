all: clean build

clean:
	-rm -rd ./frontend/dist
	cargo clean

build:
	cargo build --release

run: build
	cargo run --release

.PHONY: all clean build run
