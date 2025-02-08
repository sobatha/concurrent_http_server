use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// test with:
// curl http://127.0.0.1:7878 -w "\nTotal time: %{time_total} seconds\n"

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    while let Ok((stream, _)) = listener.accept() {
        handle_connection(stream)
    }
}

fn handle_connection(stream: TcpStream) {
    let stream1 = stream.try_clone().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream1);

    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    println!("Received request: {}", request_line.trim_end());

    // heavy process happening
    thread::sleep(Duration::from_secs(5));

    let body = "Hello";

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    );

    // Write the response to the stream and flush
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}
