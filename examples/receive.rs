extern crate udp_polygon;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use udp_polygon::{config::Config, config::FromArguments, consume};

fn main() {
    let config = Config::from_arguments(
        vec![(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5060)],
        (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5061),
    );

    let (mut socket, mut buffer) = udp_polygon::configure(config);

    consume(&mut socket, &mut buffer);
}
