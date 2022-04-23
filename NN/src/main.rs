use ipconfig::*;
use local_ip_address::*;
fn main() {
    println!("Hello, world!");
    for i in 0..255 as u8 {
        print!("{}{:?}\t", i, i as char);
    }
    println!("\n");
    (65..91 as u8).for_each(|a| print!("{}{:?} ", a, a as char));
    println!("");
    (97..123 as u8).for_each(|a| print!("{}{:?} ", a, a as char));

    let my_local_ip = local_ip().unwrap();
    println!("\nThis is my local IP address: {:?}", my_local_ip);

    let network_interfaces = list_afinet_netifas().unwrap();
    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
    }

    println!("");
    for adapter in ipconfig::get_adapters().unwrap() {
        println!("Ip addresses: {:#?}", adapter.ip_addresses());
        println!("Dns servers: {:#?}", adapter.dns_servers());
    }
}
