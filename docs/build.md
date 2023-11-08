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

- (Optional) Run cargo build
  ```
  $ cargo build --no-default-features
     ...
     Compiling pyo3-ffi v0.19.2
     Compiling pyo3 v0.19.2
     Compiling python_genshin_artifact v0.1.1 (/home/kotori/projects/python/python-genshin-artifact/python_genshin_artifact_core)
      Finished dev [unoptimized + debuginfo] target(s) in 3.12s
  ```
- Use maturin to install the rust library into a python library
  ```
  $ maturin develop
  ...
  📦 Built wheel for CPython 3.11 to /tmp/.tmpicGM3M/genshin_artifact_core-0.1.1-cp311-cp311-linux_x86_64.whl
  🛠 Installed python_genshin_artifact-0.1.1
  ```
- Use maturin to build the wheel
  ```
  $ maturin build --out dist
  ...
      Finished dev [unoptimized + debuginfo] target(s) in 3.88s
  📦 Built wheel for CPython 3.11 to dist/python_genshin_artifact-0.1.4-cp311-cp311-manylinux_2_34_x86_64.whl
  ```
- Install the built wheel with pip:
  ```
  $ pip install dist/python_genshin_artifact-xxx.whl
  ```


## Usage

Refer to other docs (i.e [DamageAnalysis](DamageAnalysis.md)) for usage.
