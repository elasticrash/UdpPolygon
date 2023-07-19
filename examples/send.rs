extern crate udp_polygon;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{thread, time};
use udp_polygon::Polygon;
use udp_polygon::{config::Config, config::FromArguments};

fn main() {
    let config = Config::from_arguments(
        vec![(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5060)],
        (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5060),
    );
    println!("config {config:?}");
    let (mut socket, _buffer) = udp_polygon::configure(config);
    println!("socket {socket:?}");
    loop {
        println!("sending");
        udp_polygon::send(
            &mut socket,
            &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5060),
            &Polygon {
                id: 0,
                msg: String::from("Hello"),
            },
        );

        thread::sleep(time::Duration::from_secs(2));
    }
}
