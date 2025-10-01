python:
    ../../../python_embeded/python.exe

install:
    ../../../python_embeded/python.exe -m pip install ./src/my-super-node/target/wheels/my_super_node-0.1.0-cp312-abi3-win_amd64.whl --force-reinstall

build:
    maturin build --release -i python3.12

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