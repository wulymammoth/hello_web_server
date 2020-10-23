use std::fs;
use std::io::prelude::*; // gain access to certain traits that let us read from and write to the stream
use std::net::{TcpListener, TcpStream};

fn main() {
    // port chosen for two reasons:
    // 1. HTTP normally accepted
    // 2. 7878 is "rust typed" on a telephone
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// NOTE: the stream is mutable (even if we're just reading) because internal state may change
// - it may actually read more data than we asked for
fn handle_connection(mut stream: TcpStream) {
    // declare a buffer that is 1024 bytes in size
    let mut buffer = [0; 1024];

    // read the bytes and stick them into the mutable buffer
    stream.read(&mut buffer).unwrap();

    // transforms a string into a "byte string" using `b` prefix on double quotes
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        // the following converts the bytes in the buffer into a string
        // the "lossy" part indicates it'll replace any invalid UTF-8 sequence with
        // a ? symbol (U+FFFD REPLACEMENT CHARACTER)
        // `let out = String::from_utf8_lossy(&buffer[..]);`
        // `println!("Request: {}", out);`
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        // convert string to bytes and write to stream
        // because the write operation could fail, we use unwrap
        stream.write(response.as_bytes()).unwrap();

        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
