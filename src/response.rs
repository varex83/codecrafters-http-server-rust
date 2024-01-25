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
    pub body: Option<ResponseBody>,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![format!("HTTP/1.1 {} OK", self.status)].concat();

        if let Some(body) = &self.body {
            lines = format!("{}\r\n{}", lines, body);
        }

        write!(f, "{}\r\n\r\n", lines)
    }
}

impl HttpResponse {
    pub fn new(status: StatusCode, body: Option<ResponseBody>) -> Self {
        HttpResponse { status, body }
    }
}

#[derive(Debug, Default)]
pub struct ResponseBody {
    pub content_type: String,
    pub content: String,
}

impl Display for ResponseBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.content_type,
            self.content.len(),
            self.content
        )
    }
}

impl TryFrom<&str> for ResponseBody {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Self {
            content_type: "text/plain".to_string(),
            content: value.to_string(),
        })
    }
}

impl ResponseBody {
    pub fn with_content_type(mut self, content_type: &str) -> Self {
        self.content_type = content_type.to_string();
        self
    }
}
