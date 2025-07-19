[![License](https://img.shields.io/crates/l/csv-line.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/csv-line.svg)](https://crates.io/crates/csv-line)
[![Documentation](https://docs.rs/csv-line/badge.svg)](https://docs.rs/csv-line)

<!-- cargo-sync-readme start -->

Fast deserialization of a single csv line.

## Usage

```rust
#[derive(Debug, PartialEq, serde::Deserialize)]
struct Foo(String, i32);

assert_eq!(
    csv_line::from_str::<Foo>("foo,42").unwrap(),
    Foo("foo".into(), 42)
);
assert_eq!(
    csv_line::from_str_sep::<Foo>("foo 42", b' ').unwrap(),
    Foo("foo".into(), 42)
);
```

## Speed

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

<!-- cargo-sync-readme end -->

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Contributing

We appreciate all kinds of contributions, thank you!

### Note on README

Most of the readme is automatically copied from the crate documentation by
[cargo-readme-sync][]. This way the readme is always in sync with the docs and
examples are tested.

So if you find a part of the readme you'd like to change between
`<!-- cargo-sync-readme start -->` and `<!-- cargo-sync-readme end -->` markers,
don't edit `README.md` directly, but rather change the documentation on top of
`src/lib.rs` and then synchronize the readme with:

```bash
cargo sync-readme
```

(make sure the cargo command is installed):

```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on
commit.

## License

This project is licensed under the [MIT license](LICENSE).

[cargo-readme-sync]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
