use std::str::FromStr;

use crate::error::Error;

pub enum TransformFormat {
    Json,
    Csv,
}

impl FromStr for TransformFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(TransformFormat::Json),
            "csv" => Ok(TransformFormat::Csv),
            _ => Err(Error::InvalidTransformFormat),
        }
    }
}
