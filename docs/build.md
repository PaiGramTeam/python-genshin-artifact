# Dev Setup

## Prerequisite

- Install [rust](https://www.rust-lang.org/tools/install)
- Confirm installation
  ```
  $ rustup --version
  rustup 1.26.0 (5af9b9484 2023-04-05)
  info: This is the version for the rustup toolchain manager, not the rustc compiler.
  info: The currently active `rustc` version is `rustc 1.75.0-nightly (1c05d50c8 2023-10-21)`
  $ cargo --version
  cargo 1.73.0 (9c4383fb5 2023-08-26)
  ```
- Install [maturin](https://github.com/PyO3/maturin) from PyO3
  ```
  $ pip install maturin
  ```


## Build

- Navigate to core -> run cargo build
  ```
  python_genshin_artifact_core
  cargo build
  ...
  ```
- Use maturin to install the rust library into a python library
  ```
  $ maturin develop
  ...
  📦 Built wheel for CPython 3.11 to /tmp/.tmpicGM3M/genshin_artifact_core-0.1.1-cp311-cp311-linux_x86_64.whl
  🛠 Installed genshin_artifact_core-0.1.1
  ```
- Navigate back to project root and build with poetry:
  ```
  $ poetry build
  Building Python-Genshin-Artifact (0.1.1)
    - Building sdist
    - Built python_genshin_artifact-0.1.1.tar.gz
    - Building wheel
    - Built python_genshin_artifact-0.1.1-py3-none-any.whl
  ```
- Install the built wheel with pip:
  ```
  $ pip install dist/python_genshin_artifact-0.1.1-py3-none-any.whl
  ```


## Usage

Refer to other docs (i.e [DamageAnalysis](DamageAnalysis.md)) for usage.
