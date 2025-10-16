build:
    maturin build --release -m ./packages/comfy-rusty-nodes/Cargo.toml -i python3.12

docker:
    just build
    docker compose up -d
    docker compose restart
    docker compose logs -f

clippy:
    cargo clippy --fix --allow-dirty
    cargo fmt

test:
    cargo test --release

publish:
    cargo ws version --no-git-push --no-git-tag
    cargo publish -p comfy-builder-macros
    cargo publish -p comfy-builder-core
#    cargo publish -p comfy-builder-custom-nodes
