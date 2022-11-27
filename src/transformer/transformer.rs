use crate::{
    decoder::Record,
    error::{Error, Result},
};

use super::TransformFormat;

type RecordIter = Box<dyn std::iter::Iterator<Item = Result<Record>>>;

pub struct Transformer {
    inner: RecordIter,
    format: TransformFormat,
}

impl Transformer {
    pub fn new(inner: RecordIter, format: TransformFormat) -> Self {
        Self { inner, format }
    }
}

impl Iterator for Transformer {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let record = match self.inner.next() {
            Some(Ok(record)) => record,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        };

        match self.format {
            TransformFormat::Json => match serde_json::to_vec(&record) {
                Ok(json) => Some(Ok(json)),
                Err(e) => Some(Err(Error::JsonSerializationFailed(e))),
            },
            TransformFormat::Csv => unimplemented!(),
        }
    }
}
