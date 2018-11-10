use std::process;
use std::io;
use std::net;
use std::time;
use pnet::packet::Packet;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::icmp::IcmpType;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::icmp::checksum;
use pnet::packet::icmp::destination_unreachable::DestinationUnreachablePacket;
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::transport::icmp_packet_iter;
use pnet::transport::TransportReceiver;
use pnet::transport::TransportSender;

use ping::PingResponse;
use ping::icmp_transport;

#[derive(Clone,Debug)]
pub struct PingRequest {
    address: net::IpAddr,
    timeout: time::Duration,
    ttl: u64,
    id: u32,
    sequence_number: u64
}

impl Default for PingRequest {
    fn default() -> Self {
        PingRequest {
            address: net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
            timeout: time::Duration::new(1u64, 0u32),
            ttl: 64u64,
            id: process::id(),
            sequence_number: 1u64
        }
    }
}

impl PingRequest {

    pub fn with_timeout(mut self, t: time::Duration) -> Self {
        self.timeout = t;
        self
    }

    pub fn timeout(&self) -> time::Duration {
        self.timeout
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

    pub fn with_sequence_number(mut self, n: u64) -> Self {
        self.sequence_number = n;
        self
    }

    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }


    pub fn with_address(mut self, a: net::IpAddr) -> Self {
        self.address = a;
        self
    }

    pub fn address(&self) -> net::IpAddr {
        self.address
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn ping(self) -> io::Result<PingResponse> {
        let (mut tx, mut rx) = icmp_transport();
        self.ping_on_channel(&mut tx, &mut rx)
    }

    pub fn ping_on_channel(self, tx: &mut TransportSender, rx: &mut TransportReceiver) -> io::Result<PingResponse> {
        let mut buffer : [u8; 64] = [3u8; 64];
        let mut request = match MutableEchoRequestPacket::new(&mut buffer) {
            Some(p) => p,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Can not create echo requesst packet"))
        };
        request.set_icmp_type(IcmpType(8));
        request.set_identifier(self.id as u16);

        let mut packet_iter = icmp_packet_iter(rx);
        request.set_sequence_number(self.sequence_number() as u16);
        
        let cs = checksum(&IcmpPacket::new(request.packet()).unwrap());
        request.set_checksum(cs);

        let new_request = request.to_immutable();
        match tx.send_to(new_request, self.address) {
            Err(e) => return Err(e),
            _ => {}
        };
        let t0 = time::Instant::now();

        // Wait for response and analyze result
        match packet_iter.next_with_timeout(self.timeout()) {
            Ok(Some((packet, addr))) => {
                let duration = time::Instant::now().duration_since(t0);
                let icmp_packet = IcmpPacket::new(packet.packet().clone()).unwrap();
                let response = match icmp_packet.get_icmp_type() {
                    IcmpTypes::EchoReply => {
                        PingResponse::new(self, addr, Some(duration))
                    },
                    IcmpTypes::DestinationUnreachable => {
                        let packet_unreach = DestinationUnreachablePacket::new(packet.packet().clone());
                        PingResponse::new(self, addr, None)
                    },
                    IcmpTypes::TimeExceeded => {
                        PingResponse::new(self, addr, None)
                    },
                    _ => {
                        PingResponse::new(self, addr, None)
                    }
                };
                Ok(response)
            },
            Ok(None) => {
                let a = self.address.clone();
                Ok(PingResponse::new(self, a, None))
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::net;
    use ping::PingRequest;

    #[test]
    fn test_ping() {
        PingRequest::default().with_address(
            net::IpAddr::V4(net::Ipv4Addr::new(8,8,8,8))
        ).ping();
    }

}


  
