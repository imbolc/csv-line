[![License](https://img.shields.io/crates/l/csv-line.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/csv-line.svg)](https://crates.io/crates/csv-line)
[![Documentation](https://docs.rs/csv-line/badge.svg)](https://docs.rs/csv-line)

# csv-line

Fast deserialization of a single csv line.

Usage
-----
```rust
#[derive(Debug, PartialEq, serde::Deserialize)]
struct Foo(String, i32);

assert_eq!(csv_line::from_str::<Foo>("foo,42").unwrap(), Foo("foo".into(), 42));
```

Speed
-----
The performance is comparable with [serde_json] (lower is better):
```bench
test csv_builder ... bench:      16,003 ns/iter (+/- 914)
test csv_core    ... bench:      15,695 ns/iter (+/- 1,155)
test csv_line    ... bench:         240 ns/iter (+/- 14)
test serde_json  ... bench:         124 ns/iter (+/- 5)
```
The benchmark code is [here][bench].

[serde_json]: https://github.com/serde-rs/json
[bench]: https://github.com/imbolc/csv-line/blob/main/benches/csv-line.rs

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
