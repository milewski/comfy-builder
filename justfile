python:
    ../../../python_embeded/python.exe

install:
    ../../../python_embeded/python.exe -m pip install ./src/my-super-node/target/wheels/my_super_node-0.1.0-cp312-abi3-win_amd64.whl --force-reinstall

build:
    cd src/my-super-node && maturin build --release -i python3.12

try:
    just build
    just install