//! Fast deserialization of a single csv line.
//!
//! Usage
//! -----
//! ```
//! #[derive(Debug, PartialEq, serde::Deserialize)]
//! struct Foo(String, i32);
//!
//! assert_eq!(csv_line::from_str::<Foo>("foo,42").unwrap(), Foo("foo".into(), 42));
//! assert_eq!(csv_line::from_str_sep::<Foo>("foo 42", ' ').unwrap(), Foo("foo".into(), 42));
//! ```
//!
//! Speed
//! -----
//! The performance is comparable with [serde_json] (lower is better):
//! ```bench
//! test csv_builder ... bench:      16,003 ns/iter (+/- 914)
//! test csv_core    ... bench:      15,695 ns/iter (+/- 1,155)
//! test csv_line    ... bench:         240 ns/iter (+/- 14)
//! test serde_json  ... bench:         124 ns/iter (+/- 5)
//! ```
//! The benchmark code is [here][bench].
//!
//! [serde_json]: https://github.com/serde-rs/json
//! [bench]: https://github.com/imbolc/csv-line/blob/main/benches/csv-line.rs

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use csv::StringRecord;
use parse::CsvRow;
use serde::de::DeserializeOwned;

mod parse;

/// An error that can occur when processing CSV data
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A wrapper for `csv::Error`
    #[error(transparent)]
    Csv(#[from] csv::Error),
}

/// A type alias for `Result<T, csv_line::Error>`
pub type Result<T> = core::result::Result<T, Error>;

/// A struct to hold the parser settings
pub struct CSVLine {
    separator: char,
}

impl CSVLine {
    /// Returns a new parser initialized with the default separator
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets a new separator, the default is `,`
    pub fn with_separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }

    /// Deserializes the string
    pub fn decode_str<T: DeserializeOwned>(&self, s: &str) -> Result<T> {
        let record = StringRecord::from_iter(CsvRow::new(s, self.separator));
        Ok(record.deserialize(None)?)
    }
}

impl Default for CSVLine {
    fn default() -> Self {
        Self { separator: ',' }
    }
}

/// Deserializes the string
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    CSVLine::new().decode_str(s)
}

/// Deserialize a csv formatted &str where the separator is specified
///
/// # Arguments
///
/// * `s` - A borrowed string slice containing csv formatted data
/// * `sep` - A u8 containing the separator use to csv format `s`
///
/// # Example with whitespace as separator:
///
/// ```
/// #[derive(Debug, PartialEq, serde::Deserialize)]
/// struct Bar(Vec<u32>);
///
/// assert_eq!(csv_line::from_str_sep::<Bar>("31 42 28 97 0", ' ').unwrap(), Bar(vec![31,42,28,97,0]));
/// ```
pub fn from_str_sep<T: DeserializeOwned>(s: &str, sep: char) -> Result<T> {
    CSVLine::new().with_separator(sep).decode_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn basic() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foo(String);
        assert_eq!(from_str::<Foo>("foo").unwrap(), Foo("foo".into()));
        assert_eq!(from_str_sep::<Foo>("foo", ' ').unwrap(), Foo("foo".into()));
    }

    #[test]
    fn empty() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foo(Option<String>);
        assert_eq!(from_str::<Foo>("").unwrap(), Foo(None));
        assert_eq!(from_str_sep::<Foo>("", ' ').unwrap(), Foo(None));
    }

    #[test]
    fn types() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foo {
            text: String,
            maybe_text: Option<String>,
            num: i32,
            flag: bool,
        }
        assert_eq!(
            from_str::<Foo>(r#""foo,bar",,1,true"#).unwrap(),
            Foo {
                text: "foo,bar".into(),
                maybe_text: None,
                num: 1,
                flag: true
            }
        );
        assert_eq!(
            from_str_sep::<Foo>(r#""foo bar"  1 true"#, ' ').unwrap(),
            Foo {
                text: "foo bar".into(),
                maybe_text: None,
                num: 1,
                flag: true
            }
        );
    }

    #[test]
    fn tsv() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foo(String, String);
        assert_eq!(
            CSVLine::new()
                .with_separator('\t')
                .decode_str::<Foo>("foo\tbar")
                .unwrap(),
            Foo("foo".into(), "bar".into())
        );
    }
}
