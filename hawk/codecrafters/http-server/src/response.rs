use std::io::Write;

pub fn respond_with_200_ok<S: Write>(stream: &mut S) -> anyhow::Result<()> {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    anyhow::Ok(())
}

pub fn respond_with_text_content<S: Write>(
    stream: &mut S,
    text_content: &str,
) -> anyhow::Result<()> {
    stream.write("HTTP/1.1 200 OK\r\n".as_bytes())?;
    stream.write("Content-Type: text/plain\r\n".as_bytes())?;
    stream.write(format!("Content-Length: {}\r\n\r\n", text_content.len()).as_bytes())?;
    stream.write(format!("{}", text_content).as_bytes())?;
    stream.flush()?;
    anyhow::Ok(())
}

pub fn respond_with_octet_stream<S: Write>(stream: &mut S, file: &[u8]) -> anyhow::Result<()> {
    stream.write("HTTP/1.1 200 OK\r\n".as_bytes())?;
    stream.write("Content-Type: application/octet-stream\r\n".as_bytes())?;
    stream.write(format!("Content-Length: {}\r\n\r\n", file.len()).as_bytes())?;
    stream.write(file)?;
    stream.flush()?;
    anyhow::Ok(())
}

pub fn respond_with_201_created<S: Write>(stream: &mut S) -> anyhow::Result<()> {
    let response = "HTTP/1.1 201 Created\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    anyhow::Ok(())
}

pub fn respond_with_404_not_found<S: Write>(stream: &mut S) -> anyhow::Result<()> {
    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    anyhow::Ok(())
}
