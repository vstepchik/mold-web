# Define default recipe
default: clean build

# Clean the project
clean:
  -rm -rd ./frontend/dist
  cargo clean

# Build the project
build:
  cargo build --release

# Run the project, ensuring 'build' is executed first
run: build
  cargo run --release

