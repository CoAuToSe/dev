use std::io::{self, *};
use std::net::*;
fn main() {
    println!("Hello, world!");
    // let talker = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    // talker
    //     .connect("127.0.0.1:8080")
    //     .expect("connect function failed");
    // talker
    //     .send_to(&[1; 10], "127.0.0.1:8080")
    //     .expect("couldn't send data");
    // std::thread::sleep(std::time::Duration::new(10, 0));
    if let Ok(mut stream) = TcpStream::connect("192.168.1.206:8080") {
        println!("Connected to the server!");
        // stream
        //     .set_nonblocking(true)
        //     .expect("set_nonblocking call failed");

        let mut buf = vec![];
        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            println!("ok");
            match stream.read_to_end(&mut buf) {
                Ok(_) => break,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    // wait_for_fd();
                    // stream.write(&format!("lolai").as_bytes()).unwrap();
                    println!("lola {:?}", buf);
                }
                Err(e) => panic!("encountered IO error: {}", e),
            };
        }
        println!("bytes: {:?}", buf);
    } else {
        println!("Couldn't connect to server...");
    }
}
