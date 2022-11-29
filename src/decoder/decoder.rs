use std::collections::HashMap;

use regex::Regex;

use crate::error::Result;
use crate::input::InputFormat;

type LineIter = Box<dyn std::iter::Iterator<Item = Result<(usize, Vec<u8>)>>>;

pub struct Decoder {
    inner: LineIter,
    re: Regex,
}

impl Decoder {
    pub fn new(inner: LineIter, format: InputFormat) -> Result<Self> {
        let re = format.get_regex()?;
        Ok(Self { inner, re })
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

        let fields: Vec<String> = self
            .re
            .capture_names()
            .filter(|field| field.is_some())
            .map(|field| field.unwrap().to_string())
            .collect();

        let line = String::from_utf8(line.to_owned())
            .expect(&format!("only UTF-8 is supported at line: {}", line_no));

        let mut record = Record::new();

        for cap in self.re.captures_iter(&line) {
            for field in &fields {
                record.insert(
                    field.to_string(),
                    cap.name(field).unwrap().as_str().to_string(),
                );
            }
        }

        Some(Ok(record))
    }
}

pub type Record = HashMap<String, String>;
