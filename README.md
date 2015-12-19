# yacli - A library for creating CLI tools with a look-and-feel similiar to Cargo
[![Travis](https://img.shields.io/travis/icorderi/cli.rs.svg)](https://travis-ci.org/icorderi/cli.rs)
[![Crates.io](https://img.shields.io/crates/v/yacli.svg)](https://crates.io/crates/yacli)
[![Crates.io](https://img.shields.io/crates/l/yacli.svg)](https://github.com/icorderi/cli.rs/blob/master/LICENSE/mit.md)

## Getting Started

### Adding the dependency

To use the latest stable **yacli** [crate] add this to your `Cargo.toml`:

```toml
[dependencies.yacli]
```

or

```toml
[dependencies]
yacli = "*"
```

or if you want to link it to the GitHub repo add this instead:

```toml
[dependencies.shell]
    git = "https://github.com/icorderi/cli.rs"
```

> **Note:** For more information on handling [dependencies] check the official cargo site.

[crate]: https://crates.io/crates/yacli
[dependencies]: http://doc.crates.io/guide.html#adding-dependencies

### Importing yacli

To import **yacli** add this to your code:

```rust
extern crate yacli;
```

## Documentation

If you need help don't forget to checkout the online [documentation] for the library.

[documentation]: http://icorderi.github.io/cli.rs/doc/yacli

## Contributing

Get involved with the [issues] or submit a [PR].

[issues]: https://github.com/icorderi/cli.rs/issues
[PR]: https://github.com/icorderi/cli.rs/pulls

## License

This project is licensed under The MIT License (MIT)
* [Markdown](LICENSE/mit.md) version
* [Original](LICENSE/mit.txt) version