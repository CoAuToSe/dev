use core::num;
use std::{
    fs::{self, *},
    io::{self, *},
    net::*,
    process::*,
    str::{self, *},
    thread::{self, *},
    time::*,
};

fn main() {
    println!("Hello, world!");
    let output = if cfg!(target_os = "windows") {
        Command::new("ipconfig")
            // .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };
    // [104, 101, 108, 108, 111, 13, 10] = "hello" as Vec<u8>
    let hellohell = output.stdout;
    // println!("{:?}\n", hellohell);
    let ipconfig = String::from_utf8_lossy(&hellohell);
    let mut splited_ipconfig = ipconfig.split("Adresse IPv4");
    // splited_ipconfig.next();

    let ip_trash = if cfg!(target_os = "windows") {
        let num_to_loop = 3;
        let mut temp = None;
        for _ in 0..num_to_loop {
            // println!("{:?}",
            temp = splited_ipconfig.next();
        }
        temp.unwrap()
    } else {
        panic!("not supported for the moment (WIP)");
    };
    // println!("\n{:?}", ip_trash);
    let tmp = ip_trash.split("\r\n").next().unwrap();
    // println!("\n{:?}", tmp);
    let tmp2 = tmp.split(".");
    // println!("\n{:?}", tmp2);
    let tmp3 = tmp2.clone().count();
    // println!("\n{:?}", tmp3);
    let mut temp5 = tmp2.clone();
    for _ in 0..tmp3 - 4 {
        // println!("{:?}",
        temp5.next();
    }
    let mut tmp6 = temp5.next().unwrap().split(":");
    tmp6.next();
    let mut tmp7 = tmp6.next().unwrap().split(" ");
    tmp7.next();
    let mut vector = vec![];
    vector.push(tmp7.next().unwrap());
    vector.push(temp5.next().unwrap());
    vector.push(temp5.next().unwrap());
    vector.push(temp5.next().unwrap());
    let final_addr = format!("{}:{}", vector.join("."), 8080);
    println!("\n {:?}", final_addr);
    let listener = TcpListener::bind(final_addr).unwrap();
    // let listener = TcpListener::bind("172.20.48.1:8080").unwrap();
    // let listener = TcpListener::bind("10.47.205.70:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("{:?}", stream);
        println!("the client with the ip: {:?}", stream.peer_addr());
        println!("want to connect to your ip: {:?}", stream.local_addr());

        std::panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        let threada = std::panic::catch_unwind(|| {
            std::thread::spawn(move || {
                {
                    let mut buffer = [0; 1024];
                    let len_content = stream.read(&mut buffer).unwrap();
                    println!(
                        "{:?}",
                        String::from_utf8((&buffer[0..len_content]).to_vec())
                    );
                    let get = b"GET / HTTP/1.1\r\n";
                    let sleep = b"GET /sleep HTTP/1.1\r\n";
                    if buffer.starts_with(b"Shutdown") {
                        // std::thread::sleep(std::time::Duration::from_secs(5));
                        panic!("trying to shutdown");
                    }
                    let (status_line, filename) = if buffer.starts_with(get) {
                        ("HTTP/1.1 200 OK", "hello.html")
                    } else if buffer.starts_with(sleep) {
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        ("HTTP/1.1 200 OK", "hello.html")
                    } else {
                        ("HTTP/1.1 404 NOT FOUND", "404.html")
                    };

                    let contents = std::fs::read_to_string(filename).unwrap();

                    let response = format!(
                        "{}\r\nContent-Length: {}\r\n\r\n{}",
                        status_line,
                        contents.len(),
                        contents
                    );
                    println!("writing content");
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            })
            .join()
            .unwrap()
        });
        match threada {
            Ok(_) => (),
            Err(_) => {
                println!("trying to shutdown (loop of client \"ended\")");
                break;
            }
        };
    }

    println!("Shutting down.");
    // println!("\n{:?}", tmp7.next());
    // println!("\n{:?}", temp5.next());
    // println!("\n{:?}", temp5.next());
    // println!("\n{:?}", temp5.next());

    // println!(
    //     "\n{:?}",
    //     splited_ipconfig.next() // .next()
    // );

    // println!(
    //     "\n{:?}",
    //     splited_ipconfig.next() // .next()
    // );
}
