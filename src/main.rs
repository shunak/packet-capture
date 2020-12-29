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
    
    // get datalink channel
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()){
        Ok(Ethernet(tx,rx)=>(tx,rx),
        Ok(_) => panic!("Unhandled channel tyep"),
        Err(e) => panic!("Failed to create datalinl channel {}",e)),
    };
    loop {
        match rx.next() {
            Ok(frame) => {
                // construct ethernet frame from received data
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype(){
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&frame);
                    }
                    _ => {
                        info!("Not an IPv4 or IPv6 packet");
                    }
                }
            }
            Err(e)=>{
                error!("Failed to read: {}",e);
            }
        }
    }
}


/**
 * Construct IPv4 packet and call handler
*/
fn ipv4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()){
        match packet.get_next_level_protocol(){
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                info!("Not a TCP or UDP packet");
            }
        }
    }
}






