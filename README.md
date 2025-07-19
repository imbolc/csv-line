[![License](https://img.shields.io/crates/l/csv-line.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/csv-line.svg)](https://crates.io/crates/csv-line)
[![Documentation](https://docs.rs/csv-line/badge.svg)](https://docs.rs/csv-line)

Fast deserialization of a single CSV line.

## Usage

```rust
#[derive(Debug, PartialEq, serde::Deserialize)]
struct Foo(String, i32);

assert_eq!(
    csv_line::from_str::<Foo>("foo,42").unwrap(),
    Foo("foo".into(), 42)
);
assert_eq!(
    csv_line::from_str_sep::<Foo>("foo 42", ' ').unwrap(),
    Foo("foo".into(), 42)
);
```

## Speed

The performance is comparable to `serde_json` (lower is better):

```text
test csv_builder ... bench:      13,190.73 ns/iter (+/- 793.61)
test csv_core    ... bench:      12,840.18 ns/iter (+/- 633.12)
test csv_line    ... bench:         176.50 ns/iter (+/- 5.15)
test serde_json  ... bench:          88.24 ns/iter (+/- 2.12
```

The benchmark code is available
[here](https://github.com/imbolc/csv-line/blob/main/benches/csv-line.rs).

## Contributing

Please run [.pre-commit.sh] before submitting a pull request to ensure all checks pass.

## License

This project is licensed under the
[MIT license](https://github.com/imbolc/csv-line/blob/main/LICENSE).

[.pre-commit.sh]: https://github.com/imbolc/csv-line/blob/main/.pre-commit.sh
