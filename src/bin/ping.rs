extern crate pnetutils;
extern crate clap;

use std::net::IpAddr;
use std::time::{Duration,Instant};

use clap::{App,Arg};

use pnetutils::ping::*;
use pnetutils::util::duration_to_millis;

#[cfg(debug_assertions)]
fn ping_process_callback(p: PingResponse) {
    match p.round_trip_time() {
        Some(rtt) => println!("Response from {}: icmp_seq={} ttl=? time={} ms {:?}", 
                        p.response_addr(),
                        p.sequence_number(),
                        duration_to_millis(&rtt),
                        p),
        None => println!("Timeout for {}: icmp_seq={} ttl=? time={} ms {:?}", 
                        p.response_addr(),
                        p.sequence_number(),
                        duration_to_millis(&p.timeout()),
                        p)
        }
}

#[cfg(not(debug_assertions))]
fn ping_process_callback(p: PingResponse) {
    match p.round_trip_time() {
        Some(rtt) => println!("Response from {}: icmp_seq={} ttl=? time={} ms", 
                        p.response_addr(),
                        p.sequence_number(),
                        duration_to_millis(&rtt)),
        None => println!("Timeout for {}: icmp_seq={} ttl=? time={} ms", 
                        p.response_addr(),
                        p.sequence_number(),
                        duration_to_millis(&p.timeout()))
        }
}

fn main() {
    let args = App::new("Rust Implementation of Ping")
                          .version("1.0")
                          .author("Manuel Landesfeind <manuel@landesfeind.de>")
                          .about("Send (IPv4) ICMP ping packets")
                          .arg(Arg::with_name("count")
                               .short("c")
                               .long("count")
                               .value_name("count")
                               .help("Stop after sending this number of ECHO_REQUEST packets")
                               .takes_value(true))
                          .arg(Arg::with_name("timeout")
                               .short("W")
                               .long("timeout")
                               .help("Specifies the timeout after which packets are considered as lost")
                               .takes_value(true))
                          .arg(Arg::with_name("address")
                               .help("Specifies the address to ping")
                               .index(1)
                               .required(true))
                        .get_matches();

    // Set up the basic ping config
    let mut ping = PingRequest::default();
    let addrstring = match args.value_of("address") {
        Some(s) => s,
        None => {
            eprintln!("Missing address");
            return
        }
    };
    let addr = match addrstring.parse() {
        Ok(a) => IpAddr::V4(a),
        Err(e) => {
            eprintln!("Can not parse IP adress: {}", e);
            return
        }
    };
    ping = ping.with_address(addr.clone());

    match args.value_of("timeout") {
        Some(ts) => match ts.parse::<u64>() {
            Ok(t) => {
                ping = ping.with_timeout(Duration::new(t, 0))
            },
            Err(e) => {
                eprintln!("Error: parameter for -W | --timeout must be a number ({})", e);
                return ;
            }
        },
        None => {}
    };

    // Set up the series
    let mut ping_series = PingSeries::default();
    match args.value_of("count") {
        Some(cs) => match cs.parse::<u64>() {
            Ok(c) => {
                ping_series = ping_series.with_number_of_packets(c)
            },
            Err(e) => {
                eprintln!("Error: parameter for -c | --count must be a number ({})", e);
                return ;
            }
        },
        None => {}
    };
    println!("PING {} ({}) with {} packets", addrstring, addr, ping_series.number_of_packets());

    let t0 = Instant::now();
    let results = match ping_series.run_with_callback(ping, ping_process_callback) {
        Ok(r) => r,
        Err(e) => panic!("{}", e)
    };
    let total_duration = Instant::now().duration_since(t0);

    let summary = PingSummary::new(results);

    println!("");
    println!("--- {} ping statistics ---", addrstring);
    println!("{} packets transmitted, {} received, {}% packet loss, time {}ms", 
             summary.packets_sent(),
             summary.packets_returned(),
             summary.packet_loss_rate() * 100f64,
             duration_to_millis(&total_duration));
    if summary.packets_returned() > 0 {
        println!("rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/{:.3} ms",
                 summary.min_rtt(),
                 summary.mean_rtt(),
                 summary.max_rtt(),
                 summary.mdev_rtt());
    }
}

