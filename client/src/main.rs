// use ipconfig::*;
// use local_ip_address::*;
use std::{
    io::{self, *},
    net::*,
};

const MESSAGE1: &str = "GET / HTTP/1.1\r
Accept: */*\r
Host: 192.168.1.203\r
Accept-Encoding: gzip, deflate, br\r
Connection: keep-alive\r\n\r\n";

const MESSAGE2: &str = "GET /index.php?v=d HTTP/1.1\r
Accept: */*\r
Accept-Encoding: gzip, deflate, br\r
Connection: keep-alive\r
Referer: http://192.168.1.203/\r
Host: 192.168.1.203\r\n\r\n";
// GET / HTTP/1.1\r\nUser-Agent: PostmanRuntime/7.29.0\r\nAccept: */*\r\nPostman-Token: 23444328-0986-4dde-9a54-2727c093f94a\r\nHost: 192.168.1.188:8080\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\n\r\n
// GET / HTTP/1.1\r\nUser-Agent: PostmanRuntime/7.29.0\r\nAccept: */*\r\nPostman-Token: 60767f1a-d585-4bed-a555-5160b20b7a59\r\nHost: 192.168.1.188:8080\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\n\r\n
fn main() {
    println!("Hello, world!");
    // let client = TcpStream::connect("10.47.205.70:8080");
    let mut client = TcpStream::connect("192.168.1.203:80").unwrap();
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let mut client = TcpStream::connect(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 203)),
        80,
    ))
    .unwrap();
    // client.write(b"GET /index.php?v=d HTTP/1.1\r\n");
    // client.write(b"GET / HTTP/1.1\r\nUser-Agent: PostmanRuntime/7.29.0\r\nAccept: */*\r\nPostman-Token: 23444328-0986-4dde-9a54-2727c093f94a\r\nHost: 192.168.1.203\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\n\r\n");
    println!("{:?}", MESSAGE1);
    let some = client.write(MESSAGE1.as_bytes()).unwrap();
    // client.write(b"GET / HTTP/1.1\r\nUser-Agent: PostmanRuntime/7.29.0\r\nAccept: */*\r\nPostman-Token: 081bedea-0873-4f66-89a5-3561d9fc85fd\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\nCookie: PHPSESSID=h5gjduflbadhis4geddasnb5jiacnfl6vt7uk87b68c44ld190dcg9vv0o2c7qe4\r\nReferer: http://192.168.1.203/\r\nHost: 192.168.1.203\r\n");
    // client.write(b"Shutdown");
    let mut buffer1 = [0; 8192];
    let content1 = client.read(&mut buffer1).unwrap();
    // client.write(MESSAGE2.as_bytes());
    // let mut client = TcpStream::connect("192.168.1.203:80").unwrap();
    // std::thread::sleep(std::time::Duration::new(0, 128 * 1_000_000));
    let some2 = client.write(MESSAGE2.as_bytes()).unwrap();
    // client.write(b"GET / HTTP/1.1\r\nUser-Agent: PostmanRuntime/7.29.0\r\nAccept: */*\r\nPostman-Token: 081bedea-0873-4f66-89a5-3561d9fc85fd\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\nCookie: PHPSESSID=h5gjduflbadhis4geddasnb5jiacnfl6vt7uk87b68c44ld190dcg9vv0o2c7qe4\r\nReferer: http://192.168.1.203/\r\nHost: 192.168.1.203\r\n");
    let mut buffer2 = [0; 2 << 15];
    let content2 = client.read(&mut buffer2).unwrap();

    println!(
        "{:?} | {:?}",
        content1,
        String::from_utf8((&buffer1[0..content1]).to_vec()).unwrap()
    );
    println!(
        "{:?} | {:?}",
        content2,
        String::from_utf8((&buffer2[0..content2]).to_vec()).unwrap()
    );

    // let my_local_ip = local_ip().unwrap();

    // println!("This is my local IP address: {:?}", my_local_ip);
    // let network_interfaces = list_afinet_netifas().unwrap();

    // for (name, ip) in network_interfaces.iter() {
    //     println!("{}:\t{:?}", name, ip);
    // }

    // for adapter in ipconfig::get_adapters().unwrap() {
    //     println!("Ip addresses: {:#?}", adapter.ip_addresses());
    //     println!("Dns servers: {:#?}", adapter.dns_servers());
    // }
    println!("{:?}", 2 << 15);
}
