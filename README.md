# sha512sum-win

A Windows port of the `sha512sum` Linux command, still under heavy development. Expect API-breaking changes as development progresses. Currently maintained by [walker84837](https://github.com/walker84837).

## Description

`sha512sum-win` is a command-line utility that prints or checks SHA512 (512-bit) checksums on Windows. It provides similar functionality to the `sha512sum` command on Linux.

## Installation

Clone the repository and build the project using Rust:

```bash
git clone https://github.com/walker84837/sha512sum-win.git
cd sha512sum-win
cargo build --release
```

Ensure you have Rust installed on your system. For Rust installation instructions, visit [rustup.rs](https://rustup.rs/).

## Usage

Run `sha512sum-win` from the command line with the path to the file you want to hash:

```bash
sha512sum-win <file_path>
```

To check a checksum, use the `-c` or `--check` option:

```bash
sha512sum-win -c <checksum_file>
```

## Examples

Hash a file:

```bash
sha512sum-win myfile.txt
```

Check a checksum:

```bash
sha512sum-win -c checksums.txt
```

## Support

For help or to report issues, please open an [issue](https://github.com/walker84837/sha512sum-win/issues).

## Contributing

Contributions are welcome! Feel free to contribute, propose new features and fix code. To contribute:
  - To use some sort of algorithm or functions, prefer the standard library over reinventing the wheel.
  - Avoid using `unsafe` code, as the code should be safe.
  - Use `rustfmt --edition 2021` to format the code.
  - For big changes you'd like to make (changing libraries doesn't count), open an issue and describe why to switch (or remove it), how you'd implement it, and what is the difference between using them and not using them.

## License

This project is dual-licensed under the [Apache 2.0](LICENSE_APACHE.md) or [MIT](LICENSE_MIT.md) license.

## Project Status

Development is not very active, as [the maintainer](https://github.com/walker84837) works on it when time permits. If you're interested in contributing or taking over maintenance, please reach out.
