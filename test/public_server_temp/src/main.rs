use std::net::*;

use std::io;
use std::io::prelude::*;
use std::io::{*};
fn main() -> std::io::Result<()> {
                let mut vector = vec![];
    for i in 0..256 {
                vector.push(std::thread::spawn(move || {
                let mut penis = String::from("");
                let mut penis2 = String::from("");
                    let mut temp_errot = String::from("");
        for j in 2..256 {//to "accelerate" change this line LOL
                    // std::thread::spawn(move || {
                // let mut vector = vec![];
            for k in 0..256 {
                // vector.push(std::thread::spawn(move || {
                for l in 0..127 {
                        let listener = TcpListener::bind(format!("{}.{}.{}.{}:0",l,k,j,i));
                        match listener {
                            Ok(something) => {
                                // accept connections and process them serially
                                // for stream in listener.incoming() {
                                //     println!("{:?}",stream);
                                // }
                                // if something.take_error() == AddrNotAvailable {}
                                println!("something: {:?} | {:?}", something, format!("{}.{}.{}.{}:0",l,k,j,i));
                            },
                            Err(e) => {
                                penis = format!("{:?}", e).clone();
                                if temp_errot == "" { temp_errot = penis.clone()}
                                if penis != temp_errot { 
                                    println!("\n{:?} | {:?}\n",penis, temp_errot); 
                                    temp_errot = penis.clone(); 
                                }
                                io::stdout().write(format!("error: {:?} | {:?} | {:?}\r", e, format!("{}, {}",j,i), format!("{}.{}.{}.{}:0",l,k,j,i)).as_bytes()).unwrap();
                                // if let Some(tempa) = temp_errot.raw_os_error() { if let Some(erro)= e.raw_os_error() { if tempa== erro {temp_errot = e; println!("");}}}
                                // println!("error: {:?}",e);
                            }
                        }
                    // })
                    ;
                    // std::thread::sleep(std::time::Duration::new(0,1));
                }
                for l in 128..256 {
                        let listener = TcpListener::bind(format!("{}.{}.{}.{}:0",l,k,j,i));
                        match listener {
                            Ok(something) => {
                                // accept connections and process them serially
                                // for stream in listener.incoming() {
                                //     println!("{:?}",stream);
                                // }
                                // if something.take_error() == AddrNotAvailable {}
                                println!("\nsomething: {:?} | {:?}\n", something, format!("{}.{}.{}.{}:0",l,k,j,i));
                            },
                            Err(e) => {     
                                penis2 = format!("{:?}", e).clone();
                                if temp_errot == "" { temp_errot = penis2.clone()}
                                if penis2 != temp_errot { 
                                    println!("\n{:?} | {:?}\n",penis2, temp_errot); 
                                    temp_errot = penis2.clone(); 
                                }
                                io::stdout().write(format!("error: {:?} | {:?} | {:?}\r", e, format!("{}, {}",j,i), format!("{}.{}.{}.{}:0",l,k,j,i)).as_bytes()).unwrap();
                                // if let Some(tempa) = temp_errot.raw_os_error() { if let Some(erro)= e.raw_os_error() { if tempa== erro {temp_errot = e; println!("");}}}
                                // println!("error: {:?}",e);
                            }
                        }
                    // })
                    ;
                    // std::thread::sleep(std::time::Duration::new(0,1));
                }
                    // }));
                // std::thread::sleep(std::time::Duration::new(1,0));
            }
                // for e in vector { e.join().unwrap() }
                    // }).join().unwrap();
            // std::thread::sleep(std::time::Duration::new(1,0));
        }
                    println!("");
        // std::thread::sleep(std::time::Duration::new(1,0));
    }));}
                for e in vector { e.join().unwrap() }
    Ok(())
}
