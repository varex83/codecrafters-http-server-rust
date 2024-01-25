use anyhow::{bail, Result};

#[derive(Debug, Default, Clone)]
pub struct HttpRequest {
    pub header_line: RequestHeaderLine,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

#[derive(Debug, Default, Clone)]
pub enum RequestMethod {
    #[default]
    GET,
    POST,
}

#[derive(Debug, Default, Clone)]
pub struct RequestHeaderLine {
    pub method: RequestMethod,
    pub path: String,
    pub version: String,
}

#[derive(Debug, Default, Clone)]
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
    pub fn new(header_line: RequestHeaderLine, headers: Vec<HttpHeader>, body: String) -> Self {
        HttpRequest {
            header_line,
            headers,
            body,
        }
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers.iter().find_map(|header| {
            if header.name == name {
                Some(header.value.clone())
            } else {
                None
            }
        })
    }
}

impl TryFrom<String> for HttpRequest {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        // <method> <path> <http-version>
        // headers: <name>: <value>
        //
        // <body>

        let mut lines = value.split("\r\n");

        let header_line = RequestHeaderLine::try_from(lines.next().unwrap().to_string())?;

        let mut headers = vec![];
        let mut body = String::new();

        let mut body_started = false;
        for line in lines {
            if line.is_empty() && !body_started {
                body_started = true;
            } else if !body_started {
                headers.push(HttpHeader::try_from(line.to_string())?);
            } else {
                body += line;
            }
        }

        Ok(HttpRequest::new(header_line, headers, body))
    }
}
