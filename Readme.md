# Rust PLS File Parser

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust CI](https://github.com/yourusername/rust-pls-parser/workflows/Rust%20CI/badge.svg)](https://github.com/yourusername/rust-pls-parser/actions)

A Rust programm for parsing PLS (Playlist) files.
PLS files are commonly used to store playlists for multimedia applications.
This applications allows you to read the content of PLS files
and display it in the command line.

## Features

- Parse PLS files and extract playlist information.
- Read and manipulate the list of entries in a PLS file.

## Usage

Add this library to your `Cargo.toml`:

```toml
[dependencies]
pls_parser = "0.1"
```

## How to use via CLI

To parse your file via CLI write the command:

```cmd
cargo run <path of your pls file>
```

for testing you can use

```cmd
cargo run test_data.pls
```

test_data.pls file is already in project.

## Contributing
Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

- Fork the repository.
- Create a new branch for your feature or bug fix.
- Make your changes and ensure tests pass.
- Create a pull request with a clear description of your changes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contact
If you have questions or need assistance, feel free to open an issue.
For other inquiries, you can contact us at arkadiy.delphi@gmail.com.

Happy coding!