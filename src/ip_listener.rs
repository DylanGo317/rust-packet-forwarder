use pnet::transport::{ipv4_packet_iter, transport_channel, TransportChannelType::{Layer3}};
use std::net::{IpAddr, Ipv4Addr};
use pnet::packet::ip::{IpNextHeaderProtocol};
use std::thread;

pub fn spawn_listener(protocol: IpNextHeaderProtocol, destination_ip: IpAddr) {
    thread::spawn(move || {

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
                    
                    if let IpAddr::V4(_ipv4_addr) = addr {
                        let linux_health_check_ip = IpAddr::V4(Ipv4Addr::new(185, 125, 190, 57));
                        if packet.get_source() == linux_health_check_ip {
                            continue;
                        }

                        // TODO: Unwrap layer 3 to modify transport layer metadata to indicate if request, response, or neither.

                        println!("got ipv4 packet: {} -> {}", packet.get_source(), packet.get_destination());

                        if let Err(e) = tx.send_to(packet, destination_ip) {
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