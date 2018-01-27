use std::net::TcpListener;
use std::{env, thread};
use std::io::Write;

const MESSAGE: &'static str = r#"HTTP/1.0 200 OK
Date: Fri, 31 Dec 1999 23:59:59 GMT
Content-Type: text/html
Content-Length: 26

<body>hello, world~</body>"#;

fn main() {
    let port = env::var("PORT").expect("PORT variable must be set");
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on PORT=[{}]", port);
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(MESSAGE.as_bytes()).unwrap();
        });
    }
}
