set dotenv-load

asset_src_dir := "assets/src"
assets_src_img_dir := asset_src_dir / "img"
frontend_static_dir := "frontend/static"
image_name := "mold-web"

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

### ====== Assets

make_favicon:
  convert -background none -density 384 {{assets_src_img_dir}}/logo.svg -define "icon:auto-resize=64,32,16" "{{frontend_static_dir}}/favicon.ico"

### ====== Docker

# Helper recipe to check if the Docker image exists
image_exists:
  @docker image inspect {{image_name}} > /dev/null 2>&1 && echo "1" || echo "0"

# Build the Docker image unconditionally
build_image:
  docker buildx build -t {{image_name}} .

# Run docker-compose, build image first if it doesn't exist
compose_up:
  {{ if `just --quiet image_exists` == "0" { `just build_image` } else { `` } }}
  docker-compose -f ./compose/compose.yaml up --build --detach
