use anyhow::Context;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Default)]
pub struct HttpRequest {
    pub http_method: String,
    pub path: String,
    pub http_version: String,
    pub host: String,
    pub user_agent: String,
    pub accept_encoding: String,
    pub content_length: usize,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn new<R: Read>(reader: &mut BufReader<R>) -> anyhow::Result<Self> {
        let mut request = Self::default();

        // Parse the fist line, ex: GET /user-agent HTTP/1.1
        let mut first_line = vec![];
        reader
            .read_until(b'\n', &mut first_line)
            .context("read the first line from request stream")?;
        let mut first_line = std::str::from_utf8(&first_line).unwrap();
        first_line = &first_line[..first_line.len() - 2];
        let mut splitted_line = first_line.split(' ');
        request.http_method = splitted_line.next().context("get HTTP method")?.to_string();
        request.path = splitted_line.next().context("get path")?.to_string();
        request.http_version = splitted_line
            .next()
            .context("get HTTP version")?
            .to_string();

        // Parse headers
        loop {
            let mut line = vec![];
            reader
                .read_until(b'\n', &mut line)
                .context("read one line from request stream")?;
            let line = std::str::from_utf8(&line).unwrap();
            match &line[..line.len() - 2] {
                l if l.is_empty() => {
                    break;
                }
                l if l.starts_with("Host: ") => {
                    request.host = l["Host: ".len()..].to_string();
                }
                l if l.starts_with("User-Agent: ") => {
                    request.user_agent = l["User-Agent: ".len()..].to_string();
                }
                l if l.starts_with("Accept-Encoding: ") => {
                    request.accept_encoding = l["Accept-Encoding: ".len()..].to_string();
                }
                l if l.starts_with("Content-Length: ") => {
                    request.content_length = l["Content-Length: ".len()..]
                        .parse::<usize>()
                        .context("parse content length")?;
                }
                _ => {
                    println!("unhandled request line: {:?}", line);
                }
            }
        }

        // Parse content
        if request.content_length > 0 {
            let mut buffer = Vec::with_capacity(request.content_length);
            buffer.resize_with(request.content_length, || 0);
            reader
                .read_exact(&mut buffer)
                .context("read request content")?;
            request.body = buffer;
        }

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_one_line_request() {
        // Arrange
        let input = "GET /path HTTP/1.1\r\n\r\n";
        let mut reader = BufReader::new(Cursor::new(input));

        // Act
        let result = HttpRequest::new(&mut reader);

        // Assert
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.http_method, "GET");
        assert_eq!(request.path, "/path");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.host, "");
        assert_eq!(request.user_agent, "");
        assert_eq!(request.accept_encoding, "");
        assert_eq!(request.content_length, 0);
        assert_eq!(request.body, vec![]);
    }

    #[test]
    fn parse_request_without_content() {
        // Arrange
        let input = [
            "GET /path HTTP/1.1\r\n",
            "Host: example.com\r\n",
            "User-Agent: TestClient\r\n",
            "Accept-Encoding: gzip\r\n",
            "\r\n",
        ]
        .join("");
        let mut reader = BufReader::new(Cursor::new(input));

        // Act
        let result = HttpRequest::new(&mut reader);

        // Assert
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.http_method, "GET");
        assert_eq!(request.path, "/path");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.host, "example.com");
        assert_eq!(request.user_agent, "TestClient");
        assert_eq!(request.accept_encoding, "gzip");
        assert_eq!(request.content_length, 0);
        assert_eq!(request.body, vec![]);
    }

    #[test]
    fn parse_request_with_content() {
        // Arrange
        let input = [
            "POST /path HTTP/1.1\r\n",
            "Host: example.com\r\n",
            "User-Agent: TestClient\r\n",
            "Accept-Encoding: gzip\r\n",
            "Content-Length: 10\r\n",
            "\r\n",
            "0123456789",
        ]
        .join("");
        let mut reader = BufReader::new(Cursor::new(input));

        // Act
        let result = HttpRequest::new(&mut reader);

        // Assert
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.http_method, "POST");
        assert_eq!(request.path, "/path");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.host, "example.com");
        assert_eq!(request.user_agent, "TestClient");
        assert_eq!(request.accept_encoding, "gzip");
        assert_eq!(request.content_length, 10);
        assert_eq!(request.body, b"0123456789");
    }

    #[test]
    fn parse_request_with_header_reordered() {
        // Arrange
        let input = [
            "POST /path HTTP/1.1\r\n",
            "Content-Length: 10\r\n",
            "User-Agent: TestClient\r\n",
            "Accept-Encoding: gzip\r\n",
            "Host: example.com\r\n",
            "\r\n",
            "0123456789",
        ]
        .join("");
        let mut reader = BufReader::new(Cursor::new(input));

        // Act
        let result = HttpRequest::new(&mut reader);

        // Assert
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.http_method, "POST");
        assert_eq!(request.path, "/path");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert_eq!(request.host, "example.com");
        assert_eq!(request.user_agent, "TestClient");
        assert_eq!(request.accept_encoding, "gzip");
        assert_eq!(request.content_length, 10);
        assert_eq!(request.body, b"0123456789");
    }

    #[test]
    fn test_parse_invalid_http_request() {
        // Arrange
        let input = "INVALID_REQUEST";
        let mut reader = BufReader::new(Cursor::new(input));

        // Act
        let result = HttpRequest::new(&mut reader);

        // Assert
        assert!(result.is_err());
    }
}
