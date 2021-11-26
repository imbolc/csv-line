# csv-line

csv-line
========

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
The benchmark code is in [here][bench].

[serde_json]: https://github.com/serde-rs/json
[bench]: https://github.com/imbolc/csv-line/blob/main/benches/csv-line.rs
