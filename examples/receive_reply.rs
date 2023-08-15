extern crate udp_polygon;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::{thread, time};
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
    let delay = time::Duration::from_millis(800);

    let mut counter = 0;
    loop {
        let maybe = rx.try_recv();
        if let Ok(_data) = maybe {
            counter += 1;
            thread::sleep(delay);
            if counter % 2 == 0 {
                polygon.send(
                    serde_json::to_string(&Message {
                        id: 1,
                        msg: String::from("Cancel Timer!"),
                    })
                    .unwrap(),
                );
            }
        }
    }
}
