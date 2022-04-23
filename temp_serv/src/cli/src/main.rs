use std::net::{self, *};

fn main() {
    println!("Hello, world!");
    let socket = UdpSocket::bind("10.47.205.70:34254").expect("couldn't bind to address");
    socket
        .connect("10.47.205.70:8080")
        .expect("connect function failed");
    socket.send(&[0, 1, 2]).expect("couldn't send message");
}
