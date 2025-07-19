#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use csv::StringRecord;
use parse::CsvRow;
use serde::de::DeserializeOwned;

mod parse;

/// A parser for a single line of CSV data.
pub struct CSVLine {
    separator: char,
}

impl CSVLine {
    /// Creates a new parser with the default separator (',').
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the separator character for the parser.
    pub fn with_separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }

    /// Deserializes a string into a custom type.
    pub fn decode_str<T: DeserializeOwned>(&self, s: &str) -> Result<T, csv::Error> {
        let record = StringRecord::from_iter(CsvRow::new(s, self.separator));
        record.deserialize(None)
    }
}

impl Default for CSVLine {
    fn default() -> Self {
        Self { separator: ',' }
    }
}

/// Deserializes a string into a custom type.
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, csv::Error> {
    CSVLine::new().decode_str(s)
}

/// Deserializes a string with a custom separator.
///
/// # Arguments
///
/// * `s` - The string slice to deserialize.
/// * `sep` - The separator character.
///
/// # Example
///
/// ```
/// #[derive(Debug, PartialEq, serde::Deserialize)]
/// struct Bar(Vec<u32>);
///
/// assert_eq!(csv_line::from_str_sep::<Bar>("31 42 28 97 0", ' ').unwrap(), Bar(vec![31,42,28,97,0]));
/// ```
pub fn from_str_sep<T: DeserializeOwned>(s: &str, sep: char) -> Result<T, csv::Error> {
    CSVLine::new().with_separator(sep).decode_str(s)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

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
