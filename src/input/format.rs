use std::str::FromStr;

use regex::Regex;

use crate::error::Error;

pub enum InputFormat {
    Regex,
    Nginx,
    Apache,
    Varnish,
}

impl FromStr for InputFormat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "nginx" => Ok(Self::Nginx),
            "varnish" => Ok(Self::Varnish),
            _ => Err(Error::InvalidInputFormat),
        }
    }
}

impl TryInto<Regex> for InputFormat {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Regex, Self::Error> {
        let re = match self {
            InputFormat::Regex => todo!(),
            InputFormat::Apache => todo!(),
            InputFormat::Nginx => Regex::new(
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
            .map_err(|e| Error::InvalidRegex(e))?,
            InputFormat::Varnish => Regex::new(
                r#"(?x)
        (?P<ip>\d{1,3}\.\d{1,3}.\d{1,3}.\d{1,3}) # IP Address
        \s-\s-\s\[
        (?P<ts>.+) # timestamp
        \]
        \s"
        (?P<method>GET|POST|PUT|PATCH|TRACE|OPTIONS|PURGE) # method
        \s
        (?P<uri>http(s?)://[^\s]*) # URI
        \s
        (?P<proto>[^\\/]*) # protocol
        /
        (?P<proto_version>[^"]*) # protocol version
        "\s
        (?P<status_code>\d{3}) # HTTP status code
        \s
        (?P<bytes>\d+) # bytes read
        \s"#,
            )
            .map_err(|e| Error::InvalidRegex(e))?,
        };

        Ok(re)
    }
}
