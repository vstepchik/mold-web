set dotenv-load

asset_src_dir := "assets/src"
assets_src_img_dir := asset_src_dir / "img"
frontend_static_dir := "frontend/static"
image_name := "mold-web"

# Clean, build, and test the project
default: clean build_backend test

@check_dev_dependencies:
  echo "This is an {{arch()}} machine running {{os()}}".
  for dep in cargo docker yarn convert; do command -v $dep >/dev/null && echo "$dep - ok" || echo "$dep - not found on PATH"; done

# Clean the project
clean:
  -rm -rd ./frontend/dist
  cargo clean

# Build the project
build_backend: build_frontend
  cargo build --release

# Run the tests
test:
  cargo test --release

# Build the front-end package
build_frontend:
  yarn --cwd ./frontend install
  yarn --cwd ./frontend build

# Build and run the project
run: build_backend
  cargo run --release

### ====== Assets

make_assets: make_favicon

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

# Shutdown docker-compose
compose_down:
  docker-compose -f ./compose/compose.yaml down
