#[cfg(any(target_os = "linux"))]
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::{thread, time::Duration};

fn main() {
    let epoll_in = EpollFlags::EPOLLIN;

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();

    let listen_fd = listener.as_raw_fd();
    let event = EpollEvent::new(epoll_in, listen_fd as u64);
    epoll.add(&listener, event).unwrap();

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];

    while let Ok(nfds) = epoll.wait(&mut events, 100 as u8) {
        for n in 0..nfds {
            if events[n].data() == listen_fd as u64 {
                if let Ok((stream, _)) = listener.accept() {
                    let fd = stream.as_raw_fd();
                    let stream1 = stream.try_clone().unwrap();
                    let stream2 = stream.try_clone().unwrap();

                    let mut reader = BufReader::new(stream);
                    let mut writer = BufWriter::new(stream1);

                    fd2buf.insert(fd, (reader, writer));

                    let mut event = EpollEvent::new(epoll_in, fd as u64);
                    epoll.add(stream2, event);
                }
            } else {
                let fd = events[n].data() as RawFd;
                handle_connection(&mut fd2buf, fd);
            }
        }
    }
}

fn handle_connection(
    fd2buf: &mut HashMap<i32, (BufReader<TcpStream>, BufWriter<TcpStream>)>,
    fd: i32,
) {
    let (reader, writer) = fd2buf.get_mut(&fd).unwrap();

    println!("Received request");

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
