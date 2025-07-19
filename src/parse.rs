use std::{borrow::Cow, str::CharIndices};

#[derive(Debug)]
enum ParseState {
    NewField,
    UnquotedField,
    QuotedField,
    QuoteInQuotedField,
    UnquotedDataAfterQuotedField(usize),
}

/// CSV row
pub(crate) struct CsvRow<'a> {
    /// The line to parse
    line: &'a str,
    /// The field delimiter
    delimiter: char,
    delimiter_len: usize,
    char_indices: CharIndices<'a>,
    /// The starting position of the current column
    column_start: usize,
    /// Whether the iterator is finished
    done: bool,
    column_needs_unescaping: bool,
}

impl<'a> CsvRow<'a> {
    /// Create a new iterator
    pub(crate) fn new(line: &'a str, delimiter: char) -> Self {
        let char_indices = line.char_indices();
        Self {
            line,
            delimiter,
            delimiter_len: delimiter.len_utf8(),
            char_indices,
            column_start: 0,
            done: false,
            column_needs_unescaping: false,
        }
    }

    fn maybe_unescape(&self, start: usize, end: usize) -> Cow<'a, str> {
        let content = &self.line[start..end];
        if self.column_needs_unescaping {
            Cow::Owned(content.replace("\"\"", "\""))
        } else {
            Cow::Borrowed(content)
        }
    }

    fn format_partially_unquoted(&self, unquoted_start: usize, end: usize) -> Cow<'a, str> {
        let quoted = self.maybe_unescape(self.column_start + 1, unquoted_start - 1);
        let unquoted = &self.line[unquoted_start..end];
        Cow::Owned(format!("{quoted}{unquoted}"))
    }
}

/// An iterator over the columns of a CSV row
impl<'a> Iterator for CsvRow<'a> {
    type Item = Cow<'a, str>;

    /// Returns the next column in the row
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = ParseState::NewField;
        self.column_needs_unescaping = false;

        // Loop over the characters in the line
        loop {
            if self.done {
                return None;
            }

            let Some((ch_pos, ch)) = self.char_indices.next() else {
                // The end of the line has been reached
                self.done = true;
                return match state {
                    ParseState::NewField => {
                        // The line ended at the end of the previous column.
                        // If the previous column ended with a delimiter, add an empty column.
                        (self.line.chars().last() == Some(self.delimiter)).then(|| "".into())
                    }
                    ParseState::UnquotedField => {
                        // The line ended in an unquoted field
                        Some(self.line[self.column_start..].into())
                    }
                    ParseState::QuotedField => {
                        // The line ended in an unclosed quoted field
                        Some(self.maybe_unescape(self.column_start + 1, self.line.len()))
                    }
                    ParseState::QuoteInQuotedField => {
                        // The line ended in a properly closed quoted field
                        Some(self.maybe_unescape(self.column_start + 1, self.line.len() - 1))
                    }
                    ParseState::UnquotedDataAfterQuotedField(unquoted_start) => {
                        let column =
                            self.format_partially_unquoted(unquoted_start, self.line.len());
                        Some(column.into())
                    }
                };
            };

            match state {
                ParseState::NewField => {
                    if ch == self.delimiter {
                        // An empty column was found
                        self.column_start = ch_pos + self.delimiter_len;
                        return Some("".into());
                    }
                    if ch == '"' {
                        state = ParseState::QuotedField;
                    } else {
                        state = ParseState::UnquotedField;
                    }
                }
                ParseState::UnquotedField => {
                    if ch == self.delimiter {
                        let column = &self.line[self.column_start..ch_pos];
                        self.column_start = ch_pos + self.delimiter_len;
                        return Some(column.into());
                    }

                    if ch == '\n' || ch == '\r' {
                        let column = &self.line[self.column_start..ch_pos];
                        self.done = true;
                        return Some(column.into());
                    }
                }
                ParseState::QuotedField => {
                    if ch == '"' {
                        state = ParseState::QuoteInQuotedField;
                    }
                }
                ParseState::QuoteInQuotedField => {
                    if ch == '"' {
                        // An escaped quote was found, so continue in the quoted field.
                        self.column_needs_unescaping = true;
                        state = ParseState::QuotedField;
                        continue;
                    }

                    if ch == self.delimiter {
                        // The end of the quoted field has been reached
                        let column = self.maybe_unescape(self.column_start + 1, ch_pos - 1);
                        self.column_start = ch_pos + self.delimiter_len;
                        return Some(column);
                    }

                    if ch == '\n' || ch == '\r' {
                        // The end of the line has been reached after a quoted field.
                        let column = self.maybe_unescape(self.column_start + 1, ch_pos - 1);
                        self.done = true;
                        return Some(column);
                    }

                    // Data was found after a quoted field, so treat it as an unquoted continuation.
                    state = ParseState::UnquotedDataAfterQuotedField(ch_pos);
                }
                ParseState::UnquotedDataAfterQuotedField(unquoted_start) => {
                    if ch == self.delimiter {
                        let column = self.format_partially_unquoted(unquoted_start, ch_pos);
                        self.column_start = ch_pos + self.delimiter_len;
                        return Some(column.into());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn csv_parse_line(line: &str) -> Vec<String> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let Some(row) = rdr.records().next() else {
            return vec![];
        };
        row.unwrap().iter().map(ToOwned::to_owned).collect()
    }

    fn parse_line(line: &str) -> Vec<String> {
        let row = CsvRow::new(line, ',');
        row.into_iter().map(Cow::into_owned).collect()
    }

    fn parse_line_with_delimiter(line: &str, delimiter: char) -> Vec<String> {
        let row = CsvRow::new(line, delimiter);
        row.into_iter().map(Cow::into_owned).collect()
    }

    /// Tests the line and ensures the result matches the output of `rust_csv`.
    macro_rules! test_line {
        ($input:expr, $expected:expr) => {
            println!("=== test started for '{}' at {}", $input, line!());
            assert_eq!(
                csv_parse_line($input),
                $expected,
                concat!("rust_csv at line ", line!())
            );
            assert_eq!(
                parse_line($input),
                $expected,
                concat!("csv_line at line ", line!())
            );
        };
    }

    /// According to §1.5, if fields are not enclosed with double quotes, then
    /// double quotes may not appear inside the fields. However, this
    /// implementation follows the `rust_csv` decision to allow it.
    #[test]
    fn quotes_in_unquoted_field() {
        test_line!(r#"foo"bar"#, [r#"foo"bar"#]);
        test_line!(r#"foo""bar"#, [r#"foo""bar"#]);
    }

    // =========================================================================
    // FIELD SEPARATION, EMPTY FIELDS (RFC 4180 §2.1, §2.2, §2.3)
    // =========================================================================

    /// §2.1: Each record is on a separate line, delimited by a line break
    /// (CRLF). §2.2: Fields are separated by commas.
    /// §2.3: The last field in the record may be empty.
    #[test]
    fn basic_unquoted_fields() {
        test_line!("foo,bar", ["foo", "bar"]);
        test_line!("foo,,bar", ["foo", "", "bar"]);
        test_line!(",foo,bar", ["", "foo", "bar"]);
        test_line!("foo,bar,", ["foo", "bar", ""]);
        test_line!(",foo,", ["", "foo", ""]);
        test_line!(",", ["", ""]);
    }

    // =========================================================================
    // QUOTED FIELDS, INCLUDING EMPTY FIELDS (RFC 4180 §2.5, §2.7)
    // =========================================================================

    /// §2.5: Fields that contain commas, CR, LF, or double quotes must be
    /// quoted. §2.7: A double quote inside a quoted field is escaped as two
    /// double quotes.
    #[test]
    fn quoted_fields() {
        test_line!(r#""foo",bar"#, ["foo", "bar"]);
        test_line!(r#""foo","bar""#, ["foo", "bar"]);
        test_line!(r#""foo",,"bar""#, ["foo", "", "bar"]);
    }

    #[test]
    fn quoted_empty_fields() {
        test_line!(r#""""#, [""]);
        test_line!(r#""","bar""#, ["", "bar"]);
        test_line!(r#""foo","""#, ["foo", ""]);
        test_line!(r#"""","""#, [r#"",""#]);
        test_line!(r#""",""","#, ["", r#"","#]);
    }

    // =========================================================================
    // QUOTED FIELDS CONTAINING COMMAS (RFC 4180 §2.5)
    // =========================================================================

    #[test]
    fn quoted_with_commas() {
        test_line!(r#""foo,bar""#, ["foo,bar"]);
        test_line!(r#""foo,bar",baz"#, ["foo,bar", "baz"]);
        test_line!(r#"a,"b,c",d"#, ["a", "b,c", "d"]);
    }

    // =========================================================================
    // QUOTED FIELDS CONTAINING NEWLINES (RFC 4180 §2.5)
    // =========================================================================

    #[test]
    fn quoted_with_newlines() {
        test_line!("\"foo\nbar\"", ["foo\nbar"]);
        test_line!("\"foo\rbar\"", ["foo\rbar"]);
        test_line!("\"foo\r\nbar\"", ["foo\r\nbar"]);
        test_line!("\"line1\nline2\",next", ["line1\nline2", "next"]);
        test_line!("\"a\nb\r\nc\",x", ["a\nb\r\nc", "x"]);
    }

    // =========================================================================
    // ESCAPED DOUBLE QUOTES IN QUOTED FIELDS (RFC 4180 §2.7)
    // =========================================================================

    #[test]
    fn escaped_quotes() {
        test_line!(r#""foo""bar""#, ["foo\"bar"]);
        test_line!(r#""""#, [""]);
        test_line!(r#""""""#, ["\""]);
        test_line!(r#""""""""#, ["\"\""]);
        test_line!(r#""say ""hello"""#, ["say \"hello\""]);
        test_line!(r#""a""b""c""#, ["a\"b\"c"]);
    }

    // =========================================================================
    // WHITESPACE HANDLING (RFC 4180 §2.6)
    // =========================================================================

    /// §2.6: Spaces are considered part of a field and should not be ignored.
    #[test]
    fn whitespace_handling() {
        test_line!(" foo , bar ", [" foo ", " bar "]);
        test_line!("\" foo \",\" bar \"", [" foo ", " bar "]);
        test_line!("foo, \"bar\" ,baz", ["foo", " \"bar\" ", "baz"]);
        test_line!(r#""foo" , "bar""#, ["foo ", r#" "bar""#]);
    }

    // =========================================================================
    // LINE ENDINGS (RFC 4180 §2.4)
    // =========================================================================

    /// §2.4: Each record is separated by a line ending, ideally CRLF.
    /// For robustness, this implementation also accepts CR or LF.
    #[test]
    fn line_ending_handling() {
        test_line!("foo,bar\n", ["foo", "bar"]);
        test_line!("foo,bar\r", ["foo", "bar"]);
        test_line!("foo,bar\r\n", ["foo", "bar"]);
        test_line!("\"foo\",\"bar\"\n", ["foo", "bar"]);
    }

    // =========================================================================
    // NON-RFC BEHAVIOR & EDGE CASES
    // =========================================================================

    /// §2.6: Spaces between the closing quote and the comma or newline are not
    /// permitted. Many implementations are permissive, but this one
    /// includes the trailing space in the field.
    #[test]
    fn space_after_closing_quote_is_error() {
        test_line!(r#""foo" ,bar"#, ["foo ", "bar"]);
    }

    /// §2.4: A CR or LF in an *unquoted* field terminates the record.
    #[test]
    fn cr_in_unquoted_field_terminates_record() {
        test_line!("foo\rbar", ["foo"]);
    }
    #[test]
    fn lf_in_unquoted_field_terminates_record() {
        test_line!("foo\nbar", ["foo"]);
    }

    // =========================================================================
    // NONSTANDARD DELIMITER SUPPORT
    // =========================================================================

    /// This is not covered by RFC 4180, but it is a common extension.
    /// This test demonstrates support for semicolon-delimited fields.
    #[test]
    fn custom_delimiter() {
        assert_eq!(
            parse_line_with_delimiter("foo;bar;baz", ';'),
            ["foo", "bar", "baz"]
        );
        assert_eq!(
            parse_line_with_delimiter(r#""f;oo";bar"#, ';'),
            ["f;oo", "bar"]
        );
        assert_eq!(
            parse_line_with_delimiter(r#""f;oo";;bar"#, ';'),
            ["f;oo", "", "bar"]
        );
    }
}
