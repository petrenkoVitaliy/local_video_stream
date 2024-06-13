use std::fmt;
use std::str::FromStr;

pub enum VideoFormat {
    MP4,
}

impl VideoFormat {
    pub fn to_str(&self) -> &str {
        match self {
            VideoFormat::MP4 => "mp4",
        }
    }
}

impl FromStr for VideoFormat {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mp4" => Ok(VideoFormat::MP4),
            _ => Err(ParseEnumError),
        }
    }
}

#[derive(Debug)]
pub struct ParseEnumError;

impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value for enum")
    }
}
