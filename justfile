build:
    maturin build --release -m ./packages/comfyui-custom-nodes/Cargo.toml -i python3.14

docker:
    just build
    docker compose up -d
    docker compose restart
    docker compose logs -f

clippy:
    cargo clippy --fix --allow-dirty

try:
    just build
    just install