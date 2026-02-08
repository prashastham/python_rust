# python_rust

Test project demonstrating how to use Rust within Python via PyO3.

## What this is

This repository is a minimal example of building a Rust extension module and importing it from Python. It is intended for experimentation and learning.

## Prerequisites

- Rust toolchain (stable)
- Python 3.10+ (recommended)
- `maturin` installed

## Build and install (editable)

From the project root:

1. Create/activate your Python environment.
2. Run:
	- `maturin develop`

This builds the Rust extension and installs it into the active Python environment.

## Quick usage

```python
import python_rust

print(python_rust.sum_as_string(2, 3))  # "5"
```

## Project structure

- [Cargo.toml](Cargo.toml) — Rust crate configuration
- [src/lib.rs](src/lib.rs) — PyO3 module and functions
- [pyproject.toml](pyproject.toml) — Python packaging config (maturin)
