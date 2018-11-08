use std::net::IpAddr;
use std::process;
use std::time::Instant;
use std::time::Duration;
use std::thread::sleep;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::icmp::{MutableIcmpPacket,IcmpPacket};
use pnet::packet::icmp::IcmpType;
use pnet::packet::icmp::checksum;
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::icmp_packet_iter;
use pnet::transport::transport_channel;

use util::duration_to_millis;

#[derive(Clone,Debug)]
pub struct Ping {
    timeout: u64,
    packets: u32,
    ttl: u64,
    sleep: u64
}

impl Default for Ping {

    fn default() -> Self {
        Ping {
            timeout: 1u64,
            packets: 4u32,
            ttl: 64u64,
            sleep: 1u64
        }
    }
}

impl Ping {

    pub fn with_timeout(mut self, t: u64) -> Self {
        if t > 0 {
            self.timeout = t;
        }
        self
    }

    pub fn timeout(&self) -> u64 {
        self.timeout
    }

    pub fn with_packets(mut self, t: u32) -> Self {
        if t > 0 {
            self.packets = t;
        }
        self
    }

    pub fn packets(&self) -> u32 {
        self.packets
    }

    pub fn with_ttl(mut self, t: u64) -> Self {
        if t > 0 {
            self.ttl = t;
        }
        self
    }

    pub fn ttl(&self) -> u64 {
        self.ttl
    }

    pub fn ping(&self, addr: IpAddr) -> Vec<Option<Duration>> {
        let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));

        // Create a new transport channel, dealing with layer 4 packets on a test protocol
        // It has a receive buffer of 4096 bytes.
        let (mut tx, mut rx) = match transport_channel(1024, protocol) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => panic!(
                "An error occurred when creating the transport channel: {}",
                e
            )
        };

        let mut buffer : [u8; 64] = [3u8; 64];
        let mut request = match MutableEchoRequestPacket::new(&mut buffer) {
            Some(p) => p,
            None => panic!("Can not create packet")
        };
        request.set_icmp_type(IcmpType(8));
        request.set_identifier(process::id() as u16);

        let mut results = Vec::new();

        let mut packet_iter = icmp_packet_iter(&mut rx);
        for seq in 0 .. self.packets {
            request.set_sequence_number((1+seq) as u16);
            
            let cs = checksum(&IcmpPacket::new(request.packet()).unwrap());
            request.set_checksum(cs);

            let new_request = request.to_immutable();
            match tx.send_to(new_request, addr) {
                Err(e) => panic!("failed to send packet: {}", e),
                //_ => println!("Sent request id={} ttl={} seq={}", request.get_identifier(), 1u32, seq)
                _ => {}
            };
            let t0 = Instant::now();

            match packet_iter.next_with_timeout(self.timeout()) {
                Ok((packet, addr)) => {
                    let duration = Instant::now().duration_since(t0);

                    let echo_reply_packet = EchoReplyPacket::new(packet.packet()).unwrap();
                    println!("{} byte from {}: icmp_seq={} ttl=? time={} ms {:?}", 
                             packet.packet().len(), 
                             addr,
                             echo_reply_packet.get_sequence_number(),
                             duration_to_millis(&duration),
                             packet);

                    results.push(Some(duration));
                }
                Err(e) => {
                    results.push(None);
                    // If an error occurs, we can handle it here
                    println!("An error occurred while reading: {}", e);
                }
            }

            if self.sleep > 0 {
                sleep(Duration::new(self.sleep, 0))
            }
        }

        results
    }

}


#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::net::Ipv4Addr;
    use ping::Ping;

    #[test]
    fn test_ping() {
        Ping::default().ping(
            IpAddr::V4(Ipv4Addr::new(8,8,8,8))
        );
    }

}


  
