use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidRequestLine,
    InvalidHeader,
    Incomplete,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidRequestLine => write!(f, "invalid request line"),
            ParseError::InvalidHeader => write!(f, "invalid header"),
            ParseError::Incomplete => write!(f, "incomplete request"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse(raw: &str) -> Result<Request, ParseError> {
    let mut lines = raw.lines();
    let request_line = lines.next().ok_or(ParseError::Incomplete)?;

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidRequestLine);
    }

    let method = parts[0].to_uppercase();
    let path = parts[1].to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let (key, value) = line
            .split_once(':')
            .ok_or(ParseError::InvalidHeader)?;
        headers.insert(key.trim().to_lowercase(), value.trim().to_string());
    }

    Ok(Request {
        method,
        path,
        headers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_get() {
        let req = parse("GET /index.html HTTP/1.1\r\n\r\n").unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/index.html");
        assert!(req.headers.is_empty());
    }

    #[test]
    fn parse_with_headers() {
        let raw = "GET / HTTP/1.1\r\nHost: localhost\r\nAccept: text/html\r\n\r\n";
        let req = parse(raw).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/");
        assert_eq!(req.headers.get("host").unwrap(), "localhost");
        assert_eq!(req.headers.get("accept").unwrap(), "text/html");
    }

    #[test]
    fn parse_empty() {
        let result = parse("");
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_path() {
        let result = parse("GET\r\n\r\n");
        assert!(result.is_err());
    }
}
