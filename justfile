build:
    maturin build --release -m ./packages/comfy-builder-custom-nodes/Cargo.toml -i python3.12

docker:
    just build
    docker compose up -d
    docker compose restart
    docker compose logs -f

clippy:
    cargo clippy --fix --allow-dirty
