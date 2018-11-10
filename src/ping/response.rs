use std::net::IpAddr;
use std::time::Duration;

use ping::PingRequest;

#[derive(Clone,Debug)]
pub struct PingResponse {
    request: PingRequest,
    rtt: Option<Duration>,
    address: IpAddr
}

impl PingResponse {

    pub fn new(request: PingRequest, from: IpAddr, rtt: Option<Duration>) -> Self {
        PingResponse {
            request: request,
            address: from,
            rtt: rtt
        }
    }

    pub fn request_addr(&self) -> IpAddr {
        self.request.address()
    }

    pub fn response_addr(&self) -> IpAddr {
        self.address
    }

    pub fn is_success(&self) -> bool {
        self.rtt.is_some() &&
            self.request_addr() == self.response_addr()
    }

    pub fn rtt(&self) -> Option<Duration> {
        self.rtt
    }

    pub fn round_trip_time(&self) -> Option<Duration> {
        self.rtt()
    }

    pub fn sequence_number(&self) -> u64 {
        self.request.sequence_number()
    }

    pub fn ttl(&self) -> u64 {
        self.request.ttl()
    }

    pub fn id(&self) -> u32 {
        self.request.id()
    }

    pub fn timeout(&self) -> Duration {
        self.request.timeout()
    }
}

