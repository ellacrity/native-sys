# native-sys

Access undocumented Native API primitives and type definitions.

## Description

This crate builds on top of the [`windows-sys`][windows-sys] crate.

- Access to undocumented Native API primitives and type definitions
- Fully compatible with `no-std` environments
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
[windows-sys]: https://crates.io/crates/windows-sys
