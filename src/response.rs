use anyhow::{bail, Result};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub enum StatusCode {
    #[default]
    Ok,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StatusCode::Ok => {
                    "200"
                }
                StatusCode::NotFound => {
                    "404"
                }
            }
        )
    }
}

impl TryFrom<&str> for StatusCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "200" => Ok(Self::Ok),
            "404" => Ok(Self::NotFound),
            _ => bail!("invalid status code provided"),
        }
    }
}

#[derive(Debug, Default)]
pub struct HttpResponse {
    pub status: StatusCode,
}

impl HttpResponse {
    pub fn new(status: StatusCode) -> Self {
        HttpResponse { status }
    }
}
