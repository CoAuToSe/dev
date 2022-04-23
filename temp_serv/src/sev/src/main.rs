use std::{
    net::{self, *},
    thread::{self, *},
    time::{self, *},
};

struct LinkConnection<'a> {
    my_ip: String,
    state: StateConnection<'a>,
}

impl<'a> LinkConnection<'a> {
    fn new() -> Self {
        LinkConnection {
            my_ip: String::from("127.0.0.1"),
            state: StateConnection::UnInitialise,
        }
    }
    fn send_this(&self, package: &[u8]) -> bool {
        match &self.state {
            UnInitialise => {
                panic!("trying to send something on a connection that is not initialize")
            }
            DistanteConnection => (panic!("this connection is not localy maintened")),
            ConnectedTo => (),
            Streaming => (),
            _ => (),
        }
        return false; //verif the state of the connection before sending data
    }
    unsafe fn send_direct(&self) -> bool {
        return false; //send package without particuliar structur
    }
    fn make_durable_stream(&self) {} //allow a stream of data to be send in a long period of time
    fn initialize_transmission(&self) {}
    fn make_link_connection(&self) {}
}

enum StateConnection<'a> {
    UnInitialise,
    DistanteConnection,
    ConnectedTo(&'a LinkConnection<'a>),
    Streaming,
    // BroadCasting //send to multiple clients
}
// enum Type_obj {
//     None = 0,
//     Usize = 1,
//     Isize = 2,
//     Specific_object = 255,
// }

fn main() {
    println!("Hello, world!");
    let socket = UdpSocket::bind("10.47.205.70:8080").unwrap();
    thread::spawn(move || loop {
        let mut buf = [0; 10];
        match socket.recv(&mut buf) {
            Ok(received) => println!("received {} bytes {:?}", received, &buf[..received]),
            Err(e) => println!("recv function failed: {:?}", e),
        }
    });
    sleep(Duration::new(10, 0));
}
