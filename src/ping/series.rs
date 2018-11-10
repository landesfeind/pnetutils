use std::io::Result;
use std::time::Duration;
use std::thread::sleep;

use ping::PingRequest;
use ping::PingResponse;
use ping::icmp_transport;

#[derive(Clone,Debug)]
pub struct PingSeries {
    ping: PingRequest,
    count: u64,
    sleep: Duration
}

fn null_callback(_:PingResponse) {}

impl PingSeries {
    pub fn new(p: PingRequest) -> Self {
        PingSeries {
            ping: p,
            count: 4u64,
            sleep: Duration::new(1, 0)
        }
    }

    pub fn with_number_of_packets(mut self, n: u64) -> Self {
        self.count = n;
        self
    }

    pub fn number_of_packets(&self) -> u64 {
        self.count
    }

    pub fn run(self) -> Result<Vec<PingResponse>> {
        self.run_with_callback(null_callback)
    }

    pub fn run_with_callback(self, on_receive: fn(PingResponse)) -> Result<Vec<PingResponse>> {
        let (mut tx, mut rx) = icmp_transport();
        let mut results = Vec::new();

        for i in 0 .. self.count {
            let p = self.ping.clone().with_sequence_number(i+1);
            match p.ping_on_channel(&mut tx, &mut rx) {
                Ok(r) => {
                    on_receive(r.clone());
                    results.push(r);
                }
                Err(e) => return Err(e)
            }

            if i < self.count - 1 {
                sleep(self.sleep);
            }
        }

        Ok(results)
    }
}

