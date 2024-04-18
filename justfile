# Clean, build, and test the project
default: clean build test

# Clean the project
clean:
  -rm -rd ./frontend/dist
  cargo clean

# Build the project
build:
  cargo build --release

# Run the tests
test:
  cargo test --release

# Build the front-end package
build_fe:
  yarn --cwd ./frontend install
  yarn --cwd ./frontend build

# Build and run the project
run: build
  cargo run --release

