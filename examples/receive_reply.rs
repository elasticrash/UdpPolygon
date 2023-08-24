extern crate udp_polygon;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use udp_polygon::{config::Address, config::Config, config::FromArguments, Polygon};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: u32,
    pub msg: String,
}

#[tokio::main]
async fn main() {
    let config = Config::from_arguments(
        vec![Address {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 5060,
        }],
        Some(Address {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 5061,
        }),
    );
    let mut polygon = Polygon::configure(config);

    let rx = polygon.receive();

    loop {
        let maybe = rx.try_recv();
        if let Ok(data) = maybe {
            println!("receiving... {data:?}");
            polygon.send(
                serde_json::to_string(&Message {
                    id: 1,
                    msg: String::from("Hello there!!!"),
                })
                .unwrap(),
            );
        }
    }
}
