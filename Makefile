all: clean build

clean:
	rm -rd ./frontend/dist
	cargo clean

build:
	cargo build --release

.PHONY: all clean build
