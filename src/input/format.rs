use std::str::FromStr;

use crate::error::Error;

pub enum InputFormat {
    Regex,
    Nginx,
    Apache,
    Varnish,
}

impl FromStr for InputFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nginx" => Ok(Self::Nginx),
            _ => Err(Error::InvalidInputFormat),
        }
    }
}
