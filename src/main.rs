/*!
# HTTP server roadmap

- [x] empty HTTP response
- [ ] support async
- [ ] support no_std
*/
#![allow(dead_code)]
use std::fmt::Formatter;
use std::io::{BufRead, Write};

const INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
"#;

/**
## HTTP Response format

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

### HTTP Response status line example

HTTP/1.1 200 OK\r\n\r\n
*/
#[derive(Copy, Clone)]
#[repr(u16)]
enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl StatusCode {
    fn code(&self) -> u16 {
        *self as u16
    }

    /// http response status line need Reason-Phrase
    fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
        }
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason_phrase())
    }
}

struct HTTPResponse {
    status: StatusCode,
    content: String,
}

impl HTTPResponse {
    fn to_http_response(&self) -> String {
        format!(
            "HTTP/1.0 {status_code} {reason_phrase}\r\nContent-Length: {len}\r\n\r\n{body}",
            status_code = self.status.code(),
            reason_phrase = self.status.reason_phrase(),
            len = self.content.len(),
            body = self.content
        )
    }
}

/** 177 bytes text body
ab -n 200 -c 10 http://127.0.0.1:8701/
(debug)n=200,c=10: 23188.41
(debug)n=200,c=1 : 10426.44
(release)n=200,c=1 : 还是1万左右
(release)n=200,c=10: 还是2.3-2.4万左右(Linux上单线程每秒2.6万返回个OK是没问题的)
*/
fn handle_request(stream: std::net::TcpStream) -> std::io::Result<()> {
    let mut stream = std::io::BufReader::new(stream);
    // println!("ok");
    let req = stream.fill_buf()?;
    let len = req.len();
    stream.consume(len);

    // from_utf8_lossy遇到不合法字符会替换成乱码
    // println!("{}", String::from_utf8_lossy(request));
    // request.split(b"\r\n");

    // let status_line = if request.starts_with(b"GET / ") {
    //     StatusCode::Ok
    // } else {
    //     StatusCode::NotFound
    // };
    // let resp = HTTPResponse {
    //     status: status_line,
    //     content: INDEX_HTML.to_string(),
    // };

    let resp = "HTTP/1.0 200 OK\r\nContent-Length: 2\r\n\r\nok";

    stream.get_mut().write(resp.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8701")?;

    for stream in listener.incoming() {
        handle_request(stream?)?;
    }
    Ok(())
}
