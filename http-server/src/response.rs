pub struct Response {
    pub status_code: u16,
    pub status_text: &'static str,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: u16, content_type: &'static str, body: Vec<u8>) -> Self {
        Self {
            status_code,
            status_text: status_text(status_code),
            content_type,
            body,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.status_code,
            self.status_text,
            self.content_type,
            self.body.len()
        );
        let mut result = header.into_bytes();
        result.extend_from_slice(&self.body);
        result
    }
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_contains_status_line() {
        let resp = Response::new(200, "text/plain", b"hello".to_vec());
        let bytes = resp.to_bytes();
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("HTTP/1.1 200 OK"));
        assert!(text.contains("Content-Type: text/plain"));
        assert!(text.contains("Content-Length: 5"));
        assert!(text.contains("hello"));
    }

    #[test]
    fn response_404() {
        let resp = Response::new(404, "text/html", b"Not Found".to_vec());
        let bytes = resp.to_bytes();
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("HTTP/1.1 404 Not Found"));
    }
}
