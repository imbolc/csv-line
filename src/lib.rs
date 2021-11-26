use csv::StringRecord;
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("quick_csv")]
    QuickCsv(#[from] quick_csv::error::Error),
    #[error("csv")]
    Csv(#[from] csv::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct CSVLine {
    delimiter: u8,
}

impl CSVLine {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets a new delimiter
    pub fn delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Deserializes the string
    pub fn decode_str<T: DeserializeOwned>(&self, s: &str) -> Result<T> {
        let record = if let Some(row) = quick_csv::Csv::from_string(s)
            .delimiter(self.delimiter)
            .into_iter()
            .next()
        {
            StringRecord::from_iter(row?.columns()?)
        } else {
            StringRecord::from(vec![""])
        };
        Ok(record.deserialize(None)?)
    }
}

impl Default for CSVLine {
    fn default() -> Self {
        Self { delimiter: b',' }
    }
}

pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    CSVLine::new().decode_str(s)
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
    }

    #[test]
    fn empty() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foo(Option<String>);
        assert_eq!(from_str::<Foo>("").unwrap(), Foo(None));
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
    }

    #[test]
    fn tsv() {
        #[derive(Debug, PartialEq, Deserialize)]
        struct Foo(String, String);
        assert_eq!(
            CSVLine::new()
                .delimiter(b'\t')
                .decode_str::<Foo>("foo\tbar")
                .unwrap(),
            Foo("foo".into(), "bar".into())
        );
    }
}
