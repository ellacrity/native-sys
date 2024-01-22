# native-sys

Access undocumented Native API primitives and type definitions.

This crate uses [`windows-sys`][windows-sys] underneath the hood rather than the heavier, more "fully-featured" [`windows-rs`][windows-rs-crate] crate.

## Description

- Access to undocumented Native API primitives and type definitions
- Uses the [`windows-sys`][windows-sys] crate for `no-std` environments and faster compilation
- Feature gates allow you to only use what you need
- Headers sourced from Process Hacker's NT headers

## Contributing

Contributions are welcome! If you find a bug or want to add new features to the library, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Disclaimer

**native-sys** is provided as-is and does not guarantee compatibility with future Windows versions. Using undocumented APIs can have unintended consequences, including system instability and security vulnerabilities. Use at your own risk.

## Special Thanks

The **native-sys** crate is a fork of the [`windows-native`][windows-native] crate. Without it, this crate would not be possible. Thank you!

<!-- Links -->

[windows-native]: https://crates.io/crates/windows-native
[windows-rs]: https://crates.io/crates/windows
[windows-sys]: https://crates.io/crates/windows-sys
