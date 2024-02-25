use anyhow::Context;
use std::io::{BufReader, Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::{env, fs};
use thread_pool::ThreadPool;

use http_server_starter_rust::request::HttpRequest;
use http_server_starter_rust::response;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let served_directory = match args.get(2) {
        Some(s) => Some(s.clone()),
        None => None,
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let served_directory = served_directory.clone();
                pool.execute(move || {
                    if handle_connection(stream, served_directory).is_err() {
                        println!("fail to handle this connection");
                    };
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    anyhow::Ok(())
}

fn handle_connection<S>(mut stream: S, served_directory: Option<String>) -> anyhow::Result<()>
where
    S: Write + Read,
{
    let mut reader = BufReader::new(&mut stream);
    let request = HttpRequest::new(&mut reader).context("parse HTTP request")?;

    match &request.path[..] {
        "/" => response::respond_with_200_ok(&mut stream)?,
        s if s.starts_with("/echo/") => {
            let random_string = &s["/echo/".len()..];
            response::respond_with_text_content(&mut stream, random_string)
                .context("echo with input string")?;
        }
        s if s.starts_with("/user-agent") => {
            response::respond_with_text_content(&mut stream, &request.user_agent)
                .context("respond with user agent")?;
        }
        s if s.starts_with("/files/") => {
            let served_directory = served_directory.context("get served directory")?;
            let filename = &s["/files/".len()..];
            let mut path = PathBuf::new();
            path.push(served_directory);
            path.push(filename);
            let path = path.as_path();

            match &request.http_method[..] {
                "GET" => {
                    if let Ok(file) = fs::read(path) {
                        response::respond_with_octet_stream(&mut stream, &file)
                            .context("respond with file")?;
                    } else {
                        response::respond_with_404_not_found(&mut stream)?;
                    };
                }
                "POST" => {
                    if let Ok(_) = fs::write(path, request.body) {
                        response::respond_with_201_created(&mut stream)?;
                    } else {
                        response::respond_with_404_not_found(&mut stream)?;
                    }
                }
                _ => response::respond_with_404_not_found(&mut stream)?,
            }
        }
        _ => response::respond_with_404_not_found(&mut stream)?,
    }

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_stream::MockStream;

    #[test]
    fn test_root_path() {
        let request = ["GET / HTTP/1.1\r\n", "\r\n"].join("");
        let mut mock_stream = MockStream::new(request.as_bytes());
        assert!(handle_connection(&mut mock_stream, None).is_ok());
        let response = mock_stream.get_received();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\n\r\n");
    }

    #[test]
    fn test_get_echo() {
        let request = ["GET /echo/hello/leo HTTP/1.1\r\n", "\r\n"].join("");
        let mut mock_stream = MockStream::new(request.as_bytes());
        assert!(handle_connection(&mut mock_stream, None).is_ok());
        let response = mock_stream.get_received();
        assert_eq!(
            response,
            [
                "HTTP/1.1 200 OK\r\n",
                "Content-Type: text/plain\r\n",
                "Content-Length: 9\r\n",
                "\r\n",
                "hello/leo"
            ]
            .join("")
            .as_bytes()
        );
    }
}
