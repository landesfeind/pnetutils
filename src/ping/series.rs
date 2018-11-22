use std::io::Result;
use std::time::Duration;
use std::thread::sleep;

use ping::PingRequest;
use ping::PingResponse;
use ping::icmp_transport;

#[derive(Clone,Debug)]
pub struct PingSeries {
    count: u64,
    sleep: Duration
}

fn null_callback(_:PingResponse) {}

impl Default for PingSeries {
    fn default() -> Self {
        PingSeries {
            count: 4u64,
            sleep: Duration::new(1, 0)
        }
    }
}

impl PingSeries {

    pub fn with_sleep(mut self, s: Duration) -> Self {
        self.sleep = s;
        self
    }

    pub fn with_number_of_packets(mut self, n: u64) -> Self {
        self.count = n;
        self
    }

    pub fn number_of_packets(&self) -> u64 {
        self.count
    }

    pub fn run(self, r: PingRequest) -> Result<Vec<PingResponse>> {
        self.run_with_callback(r, null_callback)
    }

    pub fn run_with_callback(self, r: PingRequest, on_receive: fn(PingResponse)) -> Result<Vec<PingResponse>> {
        let (mut tx, mut rx) = icmp_transport();
        let mut results = Vec::new();

        for i in 0 .. self.count {
            let p = r.clone().with_sequence_number(i+1);
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

