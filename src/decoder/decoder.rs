use std::collections::HashMap;

use regex::Regex;

use crate::error::{Error, Result};
use crate::input::InputFormat;

type LineIter = Box<dyn std::iter::Iterator<Item = Result<(usize, Vec<u8>)>>>;

pub struct Decoder {
    inner: LineIter,
    format: InputFormat,
}

impl Decoder {
    pub fn new(inner: LineIter, format: InputFormat) -> Self {
        Self { inner, format }
    }
}

impl Iterator for Decoder {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let (line_no, line) = match self.inner.next() {
            Some(Ok(res)) => res,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        };

        match self.format {
            InputFormat::Regex => unimplemented!(),
            InputFormat::Nginx => {
                // TODO: use lazy static
                let re = Regex::new(
                    r##"(?x)
            ^(?P<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3})
            \s-\s-\s\[
            (?P<ts>.+)
            ]\s"
            (?P<method>GET|POST|PUT|HEAD|PATCH|TRACE|OPTIONS|PURGE)
            \s
            (?P<uri>[^\s]*)
            \s
            (?P<protocol>[^"]*)
            "\s
            (?P<status_code>\d{3})
            \s
        "##,
                )
                .map_err(|e| Error::InvalidRegex(e))
                .unwrap();

                let fields: Vec<String> = re
                    .capture_names()
                    .filter(|field| field.is_some())
                    .map(|field| field.unwrap().to_string())
                    .collect();

                let line = String::from_utf8(line.to_owned())
                    .expect(&format!("only UTF-8 is supported at line: {}", line_no));

                let mut record = Record::new();

                for cap in re.captures_iter(&line) {
                    for field in &fields {
                        record.insert(
                            field.to_string(),
                            cap.name(field).unwrap().as_str().to_string(),
                        );
                    }
                }

                Some(Ok(record))
            }
            InputFormat::Apache => unimplemented!(),
            InputFormat::Varnish => unimplemented!(),
        }
    }
}

pub type Record = HashMap<String, String>;
