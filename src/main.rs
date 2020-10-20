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
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
