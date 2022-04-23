#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(unused_variables)]
// #![derive(Debug)]
use std::{
    net::{
        TcpListener,
        TcpStream,
        SocketAddr,
        SocketAddrV4,
        Ipv4Addr,
    },
    sync::{
        Arc,
        Mutex, 
        mpsc::{ self, * },
    },
    thread::{ self, * },
    time::Duration,
    str,
    io::prelude::*,
    fs,
};
use std::alloc::System;

#[derive(Debug)]
enum Typeses {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64, f128,
}

fn main() {
    work();
    thread::sleep(Duration::new(10, 0));
    // println!("Hello, world!");
    // let emps = TcpListener::bind("127.0.0.1:8080").unwrap();
    // println!("{:?}", emps);
    // let thread = thread::new( || {for stream in emps.incoming() {
    //     match stream {
    //             Ok(mut stream) => {
    //                 println!("new client: {:?}", stream);
    //             }
    //             Err(e) => println!("couldn't get client: {:?}", e),
    //         // println!("{:?}", emps.);

    //         }
    //     }  
    // });  
    // send_to(None,None);
    // for stream in emps.incoming() {
    //     match stream {
    //             Ok(mut stream) => {
    //                 println!("new client: {:?}", stream);
    //             }
    //             Err(e) => println!("couldn't get client: {:?}", e),
    //         // println!("{:?}", emps.);

    //         }
    //     }  
}
fn work() {
    println!("Hello, world!");
    let server = thread::spawn( || {
        let emps = TcpListener::bind("127.0.0.1:8080").unwrap();
        println!("{:?}", emps);
        let mut index = 0;
        for stream in emps.incoming() {
            match stream {
                Ok(mut stream) => {
                    let my_index = index.clone();
                    println!("new client {:?}: {:?}",my_index, stream);
                    let mut soea = vec![0u8 ; 100];
                    // soea.reserve(100);
                    let buffer: &mut [u8]= &mut soea;
                    let sometimes = stream.read(buffer).unwrap();
                    stream.write(b"got it");
                    // println!("wtf {:?}", my_index);
                    assert_ne!(sometimes, 0);
                    // if sometimes != 0 {
                        let mut temp = vec![];
                        let other_azrz = {
                            for i in 0..sometimes { 
                                // println!("{:?}", buffer[i] );
                                temp.push(buffer[i]);
                            };
                            println!("the new client {:?} send: {:?}", my_index, temp);
                            match str::from_utf8(&temp[..]) {
                                Ok(v) => v,
                                Err(e) => panic!("{:?}: Invalid UTF-8 sequence: {}", my_index, e),
                            }
                        };
                        println!("the new client {:?} said: {:?}", my_index, other_azrz);
                    // }
                    thread::spawn(move || { loop {
                        let mut soea = vec![0u8 ; 100];
                        // soea.reserve(100);
                        let buffer: &mut [u8]= &mut soea;
                        let sometimes = stream.read(buffer).unwrap();
                        stream.write(b"got it");
                        assert_ne!(sometimes, 0);
                        // if sometimes != 0 {
                            let mut temp = vec![];
                            let other_azrz = {
                                for i in 0..sometimes { 
                                    // println!("{:?}", buffer[i] );
                                    temp.push(buffer[i]);
                                };
                                // match temp {
                                //     []
                                // }
                                println!("client {:?} send: {:?}", my_index, temp);
                                match str::from_utf8(&temp[..]) {
                                    Ok(v) => v,
                                    Err(e) => panic!("{:?}: Invalid UTF-8 sequence: {}", my_index, e),
                                }
                            };
                            println!("client {:?} said: {:?}", my_index, other_azrz);
                        // }
                    }});
                    index+=1;
                },
                Err(e) => eprintln!("couldn't get client: {:?}", e),
            }
        }  
    });
    thread::sleep(Duration::new(1,0));
    let sender = thread::spawn(|| {
        println!("{:?}", send_to(None,Some(String::from("lol"))));
    });
    thread::sleep(Duration::new(1,0));
    let sender = thread::spawn(|| {
        println!("{:?}", send_to(None,Some(String::from("xD"))));
    });
    // sender.join().unwrap();
    // server.join().unwrap(); // wait infinit
    thread::sleep(Duration::new(30,0));
    println!("lolend (everything after should be irrevelant)");
}


fn send_to(addrty : Option<&str>, addrty2 : Option<String>) -> std::io::Result<()> {
    let mut stream = match addrty {
        Some(art) => TcpStream::connect(art)?,
        // None => TcpStream::connect_timeout(&SocketAddr::V4( SocketAddrV4::new( Ipv4Addr::new(127,0,0,1),8080) ) , Duration::new(0,1)).unwrap(),
        None => TcpStream::connect("127.0.0.1:8080")?,
    };
    thread::sleep(Duration::new(1,0));

    let mut stream_cloned = stream.try_clone().expect("clone failed...");
    let name = addrty2.clone();
    thread::spawn(move || { loop {
        let mut temp_buf = vec![0; 100];
        // let ref_buf = &mut temp_buf;
        let len_message = stream_cloned.read(&mut temp_buf).unwrap();
        let mut temp = vec![];
        let rece = {
            for i in 0..len_message { 
                // println!("{:?}", buffer[i] );
                temp.push(temp_buf[i]);
            };
            // match temp {
            //     []
            // }
            println!("{:?} GOT: {:?}", name, temp);
            match str::from_utf8(&temp[..]) {
                Ok(v) => v,
                Err(e) => panic!("{:?}: Invalid UTF-8 sequence: {}", name, e),
            }
        };
        println!("{:?} read: {:?}", name, rece);
    }});
    // stream.set_read_timeout(None).expect("set_read_timeout call failed");
    // stream.set_write_timeout(None).expect("set_write_timeout call failed");
    // stream.set_nodelay(true).expect("set_nodelay call failed");
    // stream.set_ttl(72 as u32).expect("set_ttl call failed");
    println!("{:?}: read timeout: {:?}", addrty2, stream.read_timeout().unwrap());
    println!("{:?}: write timeout: {:?}", addrty2, stream.write_timeout().unwrap());
    println!("{:?}: nodelay: {:?}", addrty2, stream.nodelay().unwrap());
    println!("{:?}: ttl: {:?}", addrty2, stream.ttl().unwrap());

    let mut stream_clone = stream.try_clone().expect("clone failed...");
    stream.write(&['u' as u8,'i' as u8,'f' as u8] as &[u8])?;

    thread::sleep(Duration::new(20,0));
    stream.write(&['l' as u8, 'u' as u8,'i' as u8,'f' as u8] as &[u8])?;
    println!("{:?}: {:?}", addrty2, stream_clone.write(&[4u8,5u8,6u8] as &[u8]));
    // let mut temp_buf = vec![0; 100];
    // // let ref_buf = &mut temp_buf;
    // let len_message = stream.read(&mut temp_buf).unwrap();
    // let mut temp = vec![];
    // let rece = {
    //     for i in 0..len_message { 
    //         // println!("{:?}", buffer[i] );
    //         temp.push(temp_buf[i]);
    //     };
    //     // match temp {
    //     //     []
    //     // }
    //     println!("{:?} GOT: {:?}", addrty2, temp);
    //     match str::from_utf8(&temp[..]) {
    //         Ok(v) => v,
    //         Err(e) => panic!("{:?}: Invalid UTF-8 sequence: {}", addrty2, e),
    //     }
    // };
    // println!("{:?} read: {:?}", addrty2, rece);
    Ok(())
} // the stream is closed here

// fn transform(contain: Option<()>, length: usize) -> Vec<Typeses> {
//     vec![]
// }