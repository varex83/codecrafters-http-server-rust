use anyhow::{bail, Result};

#[derive(Debug, Default)]
pub struct HttpRequest {
    pub header_line: RequestHeaderLine,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
#[derive(Debug, Default)]
pub enum RequestMethod {
    #[default]
    GET,
    POST,
}

#[derive(Debug, Default)]
pub struct RequestHeaderLine {
    pub method: RequestMethod,
    pub path: String,
    pub version: String,
}

#[derive(Debug, Default)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl TryFrom<String> for HttpHeader {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let values = value
            .split(": ")
            .map(str::to_string)
            .collect::<Vec<String>>();

        if values.len() < 2 {
            bail!("invalid http header len")
        } else {
            Ok(HttpHeader {
                name: values[0].clone(),
                value: values[1].clone(),
            })
        }
    }
}

impl TryFrom<String> for RequestMethod {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            _ => bail!("failed to convert the http request method"),
        }
    }
}

impl TryFrom<String> for RequestHeaderLine {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        let values = value
            .split_ascii_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>();

        if values.len() < 3 {
            bail!("invalid http header line len")
        } else {
            Ok(RequestHeaderLine {
                method: RequestMethod::try_from(values[0].clone())?,
                path: values[1].clone(),
                version: values[2].clone(),
            })
        }
    }
}

impl HttpRequest {
    pub fn new(
        header_line: RequestHeaderLine,
        headers: Vec<(String, String)>,
        body: String,
    ) -> Self {
        HttpRequest {
            header_line,
            headers,
            body,
        }
    }
}

impl TryFrom<String> for HttpRequest {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        // <method> <path> <http-version>
        // headers: <name>: <value>

        let lines = value
            .split("\r\n")
            .map(str::to_string)
            .collect::<Vec<String>>();

        let mut lines = lines
            .iter()
            .map(|line| line.trim().to_string())
            .collect::<Vec<String>>();

        lines.pop();

        let header_line = RequestHeaderLine::try_from(lines[0].clone())?;
        let http_headers = lines[1..lines.len() - 1]
            .iter()
            .map(|line| HttpHeader::try_from(line.clone()))
            .collect::<Result<Vec<HttpHeader>>>()?;
        let body = lines[lines.len() - 1].clone();

        Ok(HttpRequest::new(
            header_line,
            http_headers
                .iter()
                .map(|header| (header.name.clone(), header.value.clone()))
                .collect::<Vec<(String, String)>>(),
            body,
        ))
    }
}
