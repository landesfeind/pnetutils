use ping::PingResponse;
use util::duration_to_millis;

#[derive(Clone,Debug)]
pub struct PingSummary {
    series: Vec<PingResponse>
}

impl PingSummary {

    pub fn new(s: Vec<PingResponse>) -> Self {
        debug_assert!(s.len() > 0);
        PingSummary {
            series: s
        }
    }

    pub fn packets_sent(&self) -> usize {
        self.series.len()
    }

    pub fn packets_returned(&self) -> usize {
        self.series.iter().filter(|r| r.is_success()).count()
    }

    pub fn packets_lost(&self) -> usize {
        self.series.iter().filter(|r| !r.is_success()).count()
    }

    pub fn packet_loss_rate(&self) -> f64 {
        (self.packets_lost() as f64) /
            (self.packets_sent() as f64)
    }

    fn success_as_millis(&self) -> Vec<f64> {
       self.series.iter()
           .filter(|r| r.is_success())
           .map(|r| duration_to_millis(&r.rtt().unwrap()))
           .collect()
    }

    pub fn min_rtt(&self) -> f64 {
        let rps = self.success_as_millis();
        let first = rps.get(0).unwrap().clone();
        rps.iter().fold(first, |acc, r| if *r < acc { *r } else { acc })
    }

    pub fn max_rtt(&self) -> f64 {
        self.success_as_millis().iter()
            .fold(0f64, |acc, r| if *r > acc { *r } else { acc })
    }

    pub fn mean_rtt(&self) -> f64 {
        let sum = self.success_as_millis().iter().fold(0f64, |acc, r| acc+r);
        sum / (self.packets_returned() as f64)
    }

    pub fn mdev_rtt(&self) -> f64 {
        let mean = self.mean_rtt();
        self.success_as_millis().iter().fold(0f64, |acc, r| acc + (r-mean))
    }

}
