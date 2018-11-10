/// Wrapper for sending ICMP pings
///
/// A single ping-test is configured through `PingRequest` 
/// and can either be executed or put into a `PingSeries` which
/// executes a number of ping tests one after the other. 
///
/// The result is represented in a `PingResponse` 


use pnet::transport::transport_channel;
use pnet::transport::TransportReceiver;
use pnet::transport::TransportSender;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::packet::ip::IpNextHeaderProtocols;

mod request;
mod response;
mod series;
mod summary;

pub use self::request::PingRequest;
pub use self::response::PingResponse;
pub use self::series::PingSeries;
pub use self::summary::PingSummary;

pub fn icmp_transport() -> (TransportSender, TransportReceiver) {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    match transport_channel(1024, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!(
            "An error occurred when creating the transport channel: {}",
            e
        )
    }
}

