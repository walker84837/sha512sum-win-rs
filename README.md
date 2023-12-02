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

Contributions are welcome! Feel free to open pull requests with improvements or bug fixes. See the [contribution guidelines](CONTRIBUTING.md) for more details.

## License

This project is dual-licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) license.

## Project Status

Development is not very active, as the maintainer works on it when time permits. If you're interested in contributing or taking over maintenance, please reach out.