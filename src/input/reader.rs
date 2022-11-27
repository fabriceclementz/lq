use std::io::BufRead;

use crate::error::{Error, Result};

pub struct LineReader<R> {
    delimiter: u8,
    line_no: usize,
    inner: R,
}

impl<R> LineReader<R> {
    pub fn new(inner: R) -> Self {
        Self::with_delimiter(inner, b'\n')
    }

    pub fn with_delimiter(inner: R, delim: u8) -> Self {
        Self {
            delimiter: delim,
            line_no: 0,
            inner,
        }
    }
}

impl<R: BufRead> Iterator for LineReader<R> {
    type Item = Result<(usize, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line_no += 1;
        let mut buf = Vec::new();

        match self.inner.read_until(self.delimiter, &mut buf) {
            Ok(bytes_read) if bytes_read == 0 => None,
            Ok(_) => Some(Ok((self.line_no, buf))),
            Err(e) => Some(Err(Error::InputReadFailed(e))),
        }
    }
}
