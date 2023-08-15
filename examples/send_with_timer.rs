extern crate udp_polygon;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::{thread, time};
use udp_polygon::timers::Timers;
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
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5061,
        }],
        Some(Address {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5060,
        }),
    );

    let mut polygon = Polygon::configure(config);

    // it is important to assign the receiver to a variable
    // else the receiver will be dropped and the thread will
    // not be able to receive any messages
    // and send with timer will not work, at it needs
    // the receiver to be alive
    let _rx = polygon.receive();

    polygon.send_with_timer(
        serde_json::to_string(&Message {
            id: 1,
            msg: String::from("Hello with timers!"),
        })
        .unwrap(),
        Timers {
            intervals: vec![500, 600, 1000, 1500],
        },
    );

    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}
