extern crate libnetutils;

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::{Duration,Instant};

use libnetutils::ping::Ping;
use libnetutils::util::duration_to_millis;


fn main() {
    //let addrstring = "192.168.178.1";
    let addrstring = "8.8.8.8";

    let addr = match addrstring.parse() {
        Ok(a) => IpAddr::V4(a),
        Err(e) => panic!("Can not parse IP adress: {}", e)
    };

    let t0 = Instant::now();
    let results = Ping::default().ping(addr);
    let total_duration = Instant::now().duration_since(t0);
    let mut results_recv : Vec<Duration> = results.iter()
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect()
        ;
    results_recv.sort();

    let n_transmitted = results.len();
    let n_received = results_recv.len();
    let p_loss = ((n_transmitted - n_received) as f64) / (n_transmitted as f64) * 100f64;

    let rtt_mean = results_recv.iter()
        .map(|val| duration_to_millis(val))
        .fold(0f64, |acc, val| acc + val) / (n_received as f64);
    let rtt_mdev = results_recv.iter()
        .map(|val| duration_to_millis(val))
        .fold(0f64, |acc, val| acc + (val - rtt_mean).abs()) / (n_received as f64);


    println!("");
    println!("--- {} ping statistics ---", addrstring);
    println!("{} packets transmitted, {} received, {}% packet loss, time {}ms", n_transmitted, n_received, p_loss, duration_to_millis(&total_duration));
    println!("rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
             duration_to_millis(results_recv.get(0).unwrap()),
             rtt_mean,
             duration_to_millis(results_recv.get(n_received-1).unwrap()),
             rtt_mdev
             );

}

