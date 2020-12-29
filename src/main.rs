extern crate pnet;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;


use std::env;

mod packets;
use packets::GettabelEndPoints;

const WIDTH::usize = 20;


fn main() {
    env::set_var("RUST_LOG", "debug");
    env_loggger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() !=2 {
        error!("Please specify target interface name")
        std::process::exit(1);
    }
    let interface_name = &args[1];

    // select interface
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interrface");

}
