[![License](https://img.shields.io/crates/l/csv-line.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/csv-line.svg)](https://crates.io/crates/csv-line)
[![Documentation](https://docs.rs/csv-line/badge.svg)](https://docs.rs/csv-line)

# {{crate}}

{{readme}}

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Contributing

We appreciate all kinds of contributions, thank you!

### Note on README

The `README.md` file isn't meant to be changed directly. It instead generated from the crate's docs
by the [cargo-readme] command:

* Install the command if you don't have it: `cargo install cargo-readme`
* Change the crate-level docs in `src/lib.rs`, or wrapping text in `README.tpl`
* Apply the changes: `cargo readme > README.md`

If you have [rusty-hook] installed the changes will apply automatically on commit.

## License

This project is licensed under the [MIT license](LICENSE).

[cargo-readme]: https://github.com/livioribeiro/cargo-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
