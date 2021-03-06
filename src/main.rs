use echo_server::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let pool = ThreadPool::new(4);
    let port = 4000;
    let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).unwrap();
    println!("listening on http://localhost:{}",port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            println!("Connection established!");
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let req = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", req);
    stream.write(req.as_bytes()).unwrap();
    stream.flush().unwrap();
}
