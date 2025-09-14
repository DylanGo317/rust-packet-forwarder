mod tcp_parser;
mod ip_listener;
mod http_test_server;

use ip_listener::spawn_listener;
use http_test_server::run_server;
use pnet::packet::ip::IpNextHeaderProtocols::{Icmp, Tcp, Udp};
use std::net::{IpAddr, SocketAddr};
use tokio::runtime::Runtime;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    test_server_ip: String,

    #[arg(long)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let ip: IpAddr = args.test_server_ip.parse().expect("Invalid IP address");
    let http_addr = SocketAddr::new(ip, args.port);


    println!("starting ip packet forwarder");

    spawn_listener(Icmp, ip);
    spawn_listener(Tcp, ip);
    spawn_listener(Udp, ip);

    // Run HTTP server in a tokio runtime
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        run_server(http_addr).await.unwrap();
    });


    loop { std::thread::sleep(std::time::Duration::from_secs(3600)); }
}
