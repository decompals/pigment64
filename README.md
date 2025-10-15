# pigment64
[![Rust](https://github.com/decompals/pigment64/actions/workflows/rust.yml/badge.svg)](https://github.com/decompals/pigment64/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/pigment64.svg)](https://crates.io/crates/pigment64)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/decompals/pigment64/blob/master/LICENSE)

pigment64 is a library written in Rust for converting N64 image data between native and png formats.

## Supported formats
pigment64 supports a variety of common N64 image formats:

- Intensity: I1, I4, I8
- Intensity + Alpha: IA4, IA8, IA16
- Color Indexed: CI4, CI8
- Direct Color: RGBA16, RGBA32

## Command line usage

pigment64 provides command-line interface (CLI):

```bash
Usage: pigment64_cli <COMMAND>

Commands:
  to-png  Converts a binary image to a PNG
  to-bin  Converts a PNG to a binary image
  help    Print this message or the help of the given subcommand(s)
```

## Library usage

To use pigment64 in your Rust project simply run a
```bash
cargo add pigment64
```

## Python bindings

pigment64 can also be used as a Python module, allowing you to integrate it into your Python scripts and tools.

You can install the Python module from PyPI:

```bash
pip install pigment64
```

### Python Binding Development

This project uses `maturin` to build the Rust-based Python extension. It is recommended to use a Python virtual environment.

#### Setup

1.  Create and activate a virtual environment:
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    ```

2.  Install dependencies:
    ```bash
    pip install maturin pytest
    ```

#### Building

To compile the library and install it into your active virtual environment, run:

```bash
maturin develop
```
## License

This project is licensed under the MIT License.