use hello::ThreadPool;
use std::io::prelude::*; // gain access to certain traits that let us read from and write to the stream
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    // port chosen for two reasons:
    // 1. HTTP normally accepted
    // 2. 7878 is "rust typed" on a telephone
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
