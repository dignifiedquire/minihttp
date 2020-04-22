use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use bytes::BytesMut;
use minihttp::{Request, Response};

async fn process(mut stream: TcpStream) -> io::Result<()> {
    let mut v = vec![0u8; 16 * 1024];
    let mut input = BytesMut::new();
    let mut output = BytesMut::new();

    loop {
        match stream.read(&mut v).await? {
            0 => return Ok(()),
            n => input.extend_from_slice(&v[..n]),
        }

        while let Some(_) = Request::decode(&mut input)? {
            Response::new()
                .header("Content-Type", "text/plain")
                .body("Hello, World!")
                .encode(&mut output);
            stream.write_all(&output).await?;
            output.clear();
        }
    }
}

fn main() -> io::Result<()> {
    use gperftools::heap_profiler::HEAP_PROFILER;
    task::block_on(async {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;
        let mut incoming = listener.incoming();

        HEAP_PROFILER
            .lock()
            .unwrap()
            .start("./async-std.hprof")
            .unwrap();

        while let Some(stream) = incoming.next().await {
            task::spawn(process(stream?));
        }
        HEAP_PROFILER.lock().unwrap().stop().unwrap();
        Ok(())
    })
}
