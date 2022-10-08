// // use std::net::UdpSocket;
// // fn main() {
// //     let socket = UdpSocket::bind("127.0.0.1:17247").expect("couldn't bind to address");
// //     socket.connect("127.0.0.1:8080").expect("connect function failed");
// //     socket.send(&[0, 1, 2]).expect("couldn't send message");
// // }
// use std::io::prelude::*;
// use std::net::TcpStream;

// fn main() -> std::io::Result<()> {
//     let mut stream = TcpStream::connect("127.0.0.1:8080")?;

//     stream.write(&[1,2,3])?;
//     stream.read(&mut [0; 128])?;
//     println!("{:?}:{:?}", stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().port());
//     Ok(())
// } // the stream is closed here

// use std::net::UdpSocket;

// fn main() {
//     let socket = UdpSocket::bind("127.0.0.1:8080").expect("couldn't bind to address");
//     // socket.connect("127.0.0.1:8080").expect("connect function failed");
//     let mut buf = [0; 10];
//     match socket.recv(&mut buf) {
//         Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
//         Err(e) => println!("recv function failed: {:?}", e),
//     }
// }


// use std::io::{self, Read};
// use std::net::TcpStream;

// fn main() {
//     let mut stream = TcpStream::connect("127.0.0.1:7878")
//         .expect("Couldn't connect to the server...");
//     stream.set_nonblocking(true).expect("set_nonblocking call failed");

//     let mut buf = vec![];
//     loop {
//         match stream.read_to_end(&mut buf) {
//             Ok(_) => break,
//             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//                 // wait until network socket is ready, typically implemented
//                 // via platform-specific APIs such as epoll or IOCP
//                 // wait_for_fd();
//             }
//             Err(e) => panic!("encountered IO error: {}", e),
//         };
//     };
//     println!("bytes: {:?}", buf);
// }
// use std::net::UdpSocket;

// fn main() {
//     let socket = UdpSocket::bind("127.0.0.1:8080").expect("couldn't bind to address");
//     // socket.connect("127.0.0.1:8080").expect("connect function failed");
//     let mut buf = [0; 10];
//     match socket.recv(&mut buf) {
//         Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
//         Err(e) => println!("recv function failed: {:?}", e),
//     }
// }


// use std::io::{self, Read};
// use std::net::TcpStream;

// fn main() {
//     let mut stream = TcpStream::connect("127.0.0.1:7878")
//         .expect("Couldn't connect to the server...");
//     stream.set_nonblocking(true).expect("set_nonblocking call failed");

//     let mut buf = vec![];
//     loop {
//         match stream.read_to_end(&mut buf) {
//             Ok(_) => break,
//             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//                 // wait until network socket is ready, typically implemented
//                 // via platform-specific APIs such as epoll or IOCP
//                 // wait_for_fd();
//             }
//             Err(e) => panic!("encountered IO error: {}", e),
//         };
//     };
//     println!("bytes: {:?}", buf);
// }

use std::{
    net::{
        TcpListener,
        TcpStream,
    },
    sync::{
        Arc,
        Mutex, 
        mpsc,
    },
    thread,
    time::Duration,
    str,
    io::prelude::*,
    fs,
    
};


fn sent_sometg(addrty : Option<&str>, addrty2 : Option<String>) -> std::io::Result<String> {
    let mut stream = match addrty {
        Some(art) => TcpStream::connect(art)?,
        // None => TcpStream::connect("127.0.0.1:8080")?,
        None => TcpStream::connect("109.215.50.59:1712")?,  // get by the command "curl ipconfig.me" // or for ipv6 "curl https://ipv6.icanhazip.com" 
        // None => TcpStream::connect("172.22.160.1:443")?,
    };
    let mut my_adret = match addrty2 {
            Some(art) => art,
            None => {
                let my_ip =  stream.local_addr().unwrap().ip();
                let my_port =  stream.local_addr().unwrap().port();
                format!("{:?}:{:?}", my_ip, my_port)
            },
        };
    println!("my_addr: {:?}",my_adret);
    let as_uadfaz = {
        let mut temp = vec![];
        for e in my_adret.bytes() {temp.push(e)}
        temp
    };

    stream.write(&as_uadfaz as &[u8])?;
    // stream.read(&mut [0; 128])?;
    // println!("{:?}:{:?}", stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().port());
    Ok(my_adret)
} // the stream is closed here

fn main() {
    let azeatea = "127.0.0.1:17172";
    let listener = TcpListener::bind(azeatea).unwrap();
    println!("{:?}", listener);
    let resulting = sent_sometg(None, Some(String::from(azeatea))).unwrap();
    println!("{:?}", resulting.clone());

    let mut index = 0;
    for stream in listener.incoming() {
        index += 1;
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let the_employe = Worker::new(index,receiver);
        match stream {
            Ok(mut stream) => {
                // let mut streamed = stream.clone();
                let mut buffer: [u8; 1024]= [0; 1024];
                let sometimes = stream.read(&mut buffer).unwrap();
                let mut temp = vec![];
                let other_azrz = {
                    for i in 0..sometimes { temp.push(buffer[i]) };
                    match str::from_utf8(&temp) {
                        Ok(v) => v,
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    }
                };
                println!("aazfezddr: {:?}", other_azrz);
                println!("new client: {:?}", stream);
                println!("{:?}", sent_sometg(Some(&other_azrz), Some(format!("{:?}:{:?}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port()))));
                println!("action: {:?}", sender.send(Message::NewJob(Box::new( || {handle_connection(stream) } )) ));
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // println!("Worker receive: {:#?}", receiver);
            let message = receiver.lock().unwrap().recv().unwrap();

            // println!("Worker receive: {:#?}", message);
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    // println!("{:?}",sent_sometg(None, None));
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}


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