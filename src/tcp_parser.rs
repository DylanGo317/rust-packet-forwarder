use pnet::packet::{Packet, MutablePacket, ipv4::{Ipv4Packet, MutableIpv4Packet}};

pub fn parse_tcp(packet: &Ipv4Packet) -> Vec<u8>{
    let mut new_buf = packet.packet().to_vec();

    let mutable_ipv4_packet = MutableIpv4Packet::new(&mut new_buf).expect("MutableIpv4Packet");

    // TODO: modify to change tcp flag to detect duplicate
    // let payload = packet.payload_mut();

    return mutable_ipv4_packet.packet().to_vec();
}