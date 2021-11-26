#![feature(test)]

extern crate test;
use serde::{de::DeserializeOwned, Deserialize};
use test::Bencher;

const CSV_STR: &str = "\"foo,bar\",,1,true\n";
const JSON_STR: &str = r#"{ "text": "foo,bar", "maybe_text": null, "num": 1, "flag": true }"#;

#[derive(Debug, PartialEq, Deserialize)]
struct Foo {
    text: String,
    maybe_text: Option<String>,
    num: i32,
    flag: bool,
}

#[bench]
fn csv_line(b: &mut Bencher) {
    let f = || csv_line::from_str::<Foo>(CSV_STR).unwrap();
    assert_eq!(f(), expected());
    b.iter(f);
}

#[bench]
fn serde_json(b: &mut Bencher) {
    let f = || serde_json::from_str::<Foo>(JSON_STR).unwrap();
    assert_eq!(f(), expected());
    b.iter(f);
}

#[bench]
fn csv_core(b: &mut Bencher) {
    let f = || csv_core_decode::<Foo>(CSV_STR);
    assert_eq!(f(), expected());
    b.iter(f);
}

#[bench]
fn csv_builder(b: &mut Bencher) {
    let f = || csv_builder_decode::<Foo>(CSV_STR);
    assert_eq!(f(), expected());
    b.iter(f);
}

fn expected() -> Foo {
    Foo {
        text: "foo,bar".into(),
        maybe_text: None,
        num: 1,
        flag: true,
    }
}

fn csv_core_decode<T: DeserializeOwned>(data: &str) -> T {
    use csv_core::Reader;

    let mut fields: Vec<Vec<u8>> = Vec::new();
    let mut output = [0; 1024];
    let mut ends = [0usize; 16];
    let input = data.as_bytes();
    let mut rdr = Reader::new();
    let (_result, _, _, num_ends) = rdr.read_record(input, &mut output, &mut ends);
    let mut start = 0;
    for end in ends {
        let field = &output[start..end];
        fields.push(field.into());
        if fields.len() == num_ends {
            break;
        }
        start += field.len();
    }

    let record = csv::ByteRecord::from(fields);
    record.deserialize(None).unwrap()
}

fn csv_builder_decode<T: DeserializeOwned>(data: &str) -> T {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    rdr.deserialize().next().unwrap().unwrap()
}
