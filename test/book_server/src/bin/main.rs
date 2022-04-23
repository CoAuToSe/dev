use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use book_server::{
    ThreadPool,
};

use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    println!("pool :\n{:#?}", pool);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("stream :\n{:#?}", stream);

        // let something: dyn book_server::NewTrait =  || {
        //     handle_connection(stream);
        // };
        pool.execute(|| { 
            handle_connection(stream); 
        });
    }

    println!("Shutting down.");
}

// #[derive(NewTrait)]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024]= [0; 1024];
    // println!("stream :\n{:#?}", stream);
    let sometimes = stream.read(&mut buffer).unwrap();
    println!("sometimes :\n{:#?}", sometimes);
    // println!("stream :\n{:#?}", stream);

    let mut temp = vec![];
    println!("buffer :\n{:#?}", {
        for i in 0..sometimes { temp.push(buffer[i]) };
        match str::from_utf8(&temp) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    });

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}