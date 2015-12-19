# rust-cli
[![Travis](https://img.shields.io/travis/icorderi/cli.rs.svg)](https://travis-ci.org/icorderi/cli.rs)
[![Crates.io](https://img.shields.io/crates/v/rust-cli.svg)](https://crates.io/crates/rust-cli)
[![Crates.io](https://img.shields.io/crates/l/rust-cli.svg)](https://github.com/icorderi/cli.rs/blob/master/LICENSE/mit.md)

Library for creating CLI tools with a look and feel similiar to Cargo

## Getting Started

### Adding the dependency

To use the latest stable **rust-cli** [crate] add this to your `Cargo.toml`:

```toml
[dependencies.rust-cli]
```

or

```toml
[dependencies]
rust-cli = "*"
```

or if you want to link it to the GitHub repo add this instead:

```toml
[dependencies.shell]
    git = "https://github.com/icorderi/cli.rs"
```

> **Note:** For more information on handling [dependencies] check the official cargo site.

[crate]: https://crates.io/crates/rust-cli
[dependencies]: http://doc.crates.io/guide.html#adding-dependencies

### Importing rust-cli

To import rust-cli add this to your code:

```rust
extern crate cli;
```

## Documentation

If you need help don't forget to checkout the online [documentation] for the library.

[documentation]: http://icorderi.github.io/cli.rs/doc/rust-cli

## Contributing

Get involved with the [issues] or submit a [PR].

[issues]: https://github.com/icorderi/cli.rs/issues
[PR]: https://github.com/icorderi/cli.rs/pulls

## License

This project is licensed under The MIT License (MIT)
* [Markdown](LICENSE/mit.md) version
* [Original](LICENSE/mit.txt) version