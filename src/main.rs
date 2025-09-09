use pnet::transport::{ipv4_packet_iter, transport_channel, TransportChannelType::{Layer3}};
use std::net::IpAddr;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols::{Icmp, Tcp, Udp}};
use std::thread;

fn spawn_listener(protocol: IpNextHeaderProtocol) {
    thread::spawn(move || {
        let local_host = std::net::Ipv4Addr::new(127,0,0,1);

        let transport_channel_type = Layer3(protocol);

        let (mut tx, mut rx) = transport_channel(65535, transport_channel_type).expect("transport_channel");

        let mut iter = ipv4_packet_iter(&mut rx);

        loop {
            match iter.next() {
                Ok((packet, addr)) => {
                    if let IpAddr::V6(_ipv6_addr) = addr {
                        println!("got ipv6 packet: {} -> {}", packet.get_source(), packet.get_destination());
                        continue;
                    }
                    
                    if let IpAddr::V4(ipv4_addr) = addr {
                        // Don't forward localhost to avoid infinite loops
                        if packet.get_source() == local_host && packet.get_destination() == local_host {
                            continue;
                        }

                        println!("got ipv4 packet: {} -> {}", packet.get_source(), packet.get_destination());

                        // Forward to intended destination
                        if let Err(e) = tx.send_to(packet, IpAddr::V4(ipv4_addr)) {
                            eprintln!("send failed: {e}");
                        }
                    }
                }

                Err(e) => {
                    eprintln!("error: {e}");
                }
            }
        }
    });
}
   

fn main() {
    println!("starting ip packet forwarder");

    spawn_listener(Icmp);
    spawn_listener(Tcp);
    spawn_listener(Udp);

    loop { std::thread::sleep(std::time::Duration::from_secs(3600)); }
}
