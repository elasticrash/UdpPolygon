extern crate udp_polygon;
use std::net::{IpAddr, Ipv4Addr};
use udp_polygon::{config::Config, config::FromArguments, Polygon};

#[tokio::main]
async fn main() {
    let config = Config::from_arguments(vec![(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5060)], None);
    let mut polygon = Polygon::configure(config);

    let rx = polygon.receive();

    loop {
        let maybe = rx.try_recv();
        if let Ok(data) = maybe {
            println!("data {data:?}");
        }
    }
}
