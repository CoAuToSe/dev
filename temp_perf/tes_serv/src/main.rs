use std::io::{self, *};
use std::net::*;
use std::sync::*;
// fn main() {
//     println!("Hello, world!");
//     let listener = TcpListener::bind("192.168.1.206:8080").unwrap();
//     let mut index = Arc::new(Mutex::new(0));
//     for stream in listener.incoming() {
//         let index = Arc::clone(&index);
//         std::thread::spawn(move || {
//             let mut stream = stream.unwrap();
//             let mut index = index.lock().unwrap();
//             println!("lol {}", index);
//             *index += 1;
//             // let mut buf = vec![];
//             // loop {
//             //     // println!("ok");
//             //     match stream.read_to_end(&mut buf) {
//             //         Ok(_) => break,
//             //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//             //             // wait until network socket is ready, typically implemented
//             //             // via platform-specific APIs such as epoll or IOCP
//             //             // wait_for_fd();
//             //             println!("lola")
//             //         }
//             //         Err(e) => panic!("encountered IO error: {}", e),
//             //     };
//             // }
//             // println!("bytes: {:?}", buf);
//             stream.write(&format!("{}", index).as_bytes()).unwrap();

//             // let mut buf = vec![];
//             // loop {
//             //     // println!("ok");
//             //     match stream.read_to_end(&mut buf) {
//             //         Ok(_) => break,
//             //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//             //             // wait until network socket is ready, typically implemented
//             //             // via platform-specific APIs such as epoll or IOCP
//             //             // wait_for_fd();
//             //             println!("lola")
//             //         }
//             //         Err(e) => panic!("encountered IO error: {}", e),
//             //     };
//             // }
//             // println!("bytes: {:?}", buf);
//         });
//     }
// }

fn main() {
    println!("Hello, world!");

    let listener = UdpSocket::bind("192.168.1.206:8080").unwrap();
    let mut index = Arc::new(Mutex::new(0));
    // for stream in listener.incoming() {
        let index = Arc::clone(&index);
        std::thread::spawn(move || {
            // let mut stream = stream.unwrap();
            let mut index = index.lock().unwrap();
            println!("lol {}", index);
            *index += 1;
            // let mut buf = vec![];
            // loop {
            //     // println!("ok");
            //     match stream.read_to_end(&mut buf) {
            //         Ok(_) => break,
            //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            //             // wait until network socket is ready, typically implemented
            //             // via platform-specific APIs such as epoll or IOCP
            //             // wait_for_fd();
            //             println!("lola")
            //         }
            //         Err(e) => panic!("encountered IO error: {}", e),
            //     };
            // }
            // println!("bytes: {:?}", buf);
            stream.write(&format!("{}", index).as_bytes()).unwrap();

            // let mut buf = vec![];
            // loop {
            //     // println!("ok");
            //     match stream.read_to_end(&mut buf) {
            //         Ok(_) => break,
            //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            //             // wait until network socket is ready, typically implemented
            //             // via platform-specific APIs such as epoll or IOCP
            //             // wait_for_fd();
            //             println!("lola")
            //         }
            //         Err(e) => panic!("encountered IO error: {}", e),
            //     };
            // }
            // println!("bytes: {:?}", buf);
        });
    }
}
