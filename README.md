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

## License

This project is licensed under the MIT License.