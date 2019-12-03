use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use bytes::BytesMut;
use minihttp::{Request, Response};

fn process(mut stream: TcpStream) -> io::Result<()> {
    let mut v = vec![0u8; 16 * 1024];
    let mut input = BytesMut::new();
    let mut output = BytesMut::new();

    loop {
        match stream.read(&mut v)? {
            0 => return Ok(()),
            n => input.extend_from_slice(&v[..n]),
        }

        while let Some(_) = Request::decode(&mut input)? {
            Response::new()
                .header("Content-Type", "text/plain")
                .body("Hello, World!")
                .encode(&mut output);
            stream.write_all(&output)?;
            output.clear();
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next() {
        let stream = stream?;
        thread::spawn(|| process(stream));
    }
    Ok(())
}
