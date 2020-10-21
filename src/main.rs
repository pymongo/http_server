/*!
# HTTP server roadmap

- [x] empty HTTP response
- [ ] support async
- [ ] support no_std
*/
use std::io::{BufRead, Write};

/**
## HTTP Response format

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

### HTTP Response example

HTTP/1.1 200 OK\r\n\r\n
*/
#[repr(u16)]
enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl StatusCode {
    fn code(&self) -> u16 {
        self as u16
    }

    fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found"
        }
    }
}

fn handle_request(stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut stream = std::io::BufReader::new(stream);
    let request = stream.fill_buf()?;
    // from_utf8_lossy遇到不合法字符会替换成乱码
    println!("{}", String::from_utf8_lossy(request));
    // TODO return same HTTP protocol from request
    stream.get_mut().write(b"HTTP/1.1 404\r\n\r\n")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8701")?;

    for stream in listener.incoming() {
        handle_request(stream?)?;
    }
    Ok(())
}
