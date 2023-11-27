//! # udp-polygon
//!
//! `udp-polygon` is a library that allows to send and receive UDP packets.
//!
//! It can be configured in many ways, using toml, args, or env vars.
//!
//! It also supports retransmission of packets, using timers.
//!
//! ## Requirements
//! * the consumer requires  [tokio](https://docs.rs/tokio/)
//! * a producer does not require anything extra
//! * a producer with the timer flag enabled requires [tokio](https://docs.rs/tokio/)

use std::net::{IpAddr, SocketAddr, UdpSocket};
/// This is the configuration module. It allows to configure the lib, using toml, args, or env vars.
pub mod config;
use config::Config;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use tokio;

#[cfg(feature = "timers")]
pub mod timers;

#[cfg(feature = "timers")]
use crate::timers::Timers;

#[cfg(feature = "timers")]
use tokio::time::Duration;

/// Polygon is a UDP socket that can send and receive data.
/// It can be configured by using the `configure` method.
/// ``` rust
/// let mut polygon = Polygon::configure(config);
/// ```
#[derive(Debug)]
pub struct Polygon {
    pub socket: UdpSocket,
    pub buffer: [u8; 65535],
    pub destination: Option<SocketAddr>,
    pub pause_timer_send: Arc<Mutex<bool>>,
}

impl Polygon {
    pub fn get_channel() -> (Sender<Vec<u8>>, Receiver<Vec<u8>>) {
        let (tx, rx) = mpsc::channel();
        (tx, rx)
    }
    pub fn configure(config: Config) -> Self {
        let addrs = config
            .bind_addresses
            .into_iter()
            .map(|addr| match addr.ip {
                IpAddr::V4(ipv4) => SocketAddr::new(IpAddr::V4(ipv4), addr.port),
                IpAddr::V6(ipv6) => SocketAddr::new(IpAddr::V6(ipv6), addr.port),
            })
            .collect::<Vec<_>>();

        let socket = match UdpSocket::bind(&addrs[..]) {
            Ok(socket) => socket,
            Err(e) => panic!("couldn't bind socket: {:?}", e),
        };

        let buffer = [0_u8; 65535];
        Self {
            socket,
            buffer,
            destination: if let Some(addr) = config.destination_address {
                match addr.ip {
                    IpAddr::V4(ipv4) => Some(SocketAddr::new(IpAddr::V4(ipv4), addr.port)),
                    IpAddr::V6(ipv6) => Some(SocketAddr::new(IpAddr::V6(ipv6), addr.port)),
                }
            } else {
                None
            },
            pause_timer_send: Arc::new(Mutex::new(false)),
        }
    }
    pub fn receive(&mut self) -> Receiver<Vec<u8>> {
        let mut socket = self.socket.try_clone().unwrap();
        let mut buffer = self.buffer;
        let (tx, rx) = Self::get_channel();

        tokio::spawn(async move {
            loop {
                let maybe: Option<Vec<u8>>;
                {
                    let packets_queued = UdpRead::peek(&mut socket, &mut buffer);
                    if packets_queued > 0 {
                        maybe = match UdpRead::read_bytes(&mut socket, &mut buffer) {
                            Ok(buf) => Some(buf),
                            Err(_) => None,
                        };

                        if let Some(data) = maybe {
                            match tx.send(data) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("receiver error: {:?}", e.to_string())
                                }
                            }
                        }
                    }
                }
            }
        });

        rx
    }

    #[cfg(feature = "timers")]
    pub fn resume_timer_send(&mut self) {
        *self.pause_timer_send.lock().unwrap() = false;
    }
    #[cfg(feature = "timers")]
    pub fn cancel_timer_receive(&mut self) {
        *self.pause_timer_send.lock().unwrap() = true;
    }
    #[cfg(feature = "timers")]
    pub fn send_with_timer(&mut self, data: Vec<u8>, timers: Timers) {
        let socket = self.socket.try_clone().unwrap();
        let destination = self.destination.clone().unwrap();
        let pause = Arc::clone(&self.pause_timer_send);
        tokio::spawn(async move {
            let mut current_timer = timers.delays.into_iter();
            let mut counter = 0;
            loop {
                if *pause.lock().unwrap() && counter > 0 {
                    break;
                }
                let next_timer = match current_timer.next() {
                    Some(timer) => timer,
                    None => {
                        break;
                    }
                };

                socket
                    .send_to(
                        &data,
                        format!("{}:{}", &destination.ip(), &destination.port()),
                    )
                    .unwrap();
                tokio::time::sleep(Duration::from_millis(next_timer)).await;
                counter += 1;
            }
        });
    }
    pub fn send(&mut self, data: Vec<u8>) {
        let destination = self.destination.unwrap();
        self.socket
            .send_to(
                &data,
                format!("{}:{}", &destination.ip(), &destination.port()),
            )
            .unwrap();
    }
    pub fn change_destination(&mut self, new_destination: SocketAddr) {
        self.destination = Some(new_destination);
    }
}

struct UdpRead;

impl UdpRead {
    fn peek(socket: &mut UdpSocket, buffer: &mut [u8; 65535]) -> usize {
        match socket.peek(buffer) {
            Ok(received) => received,
            Err(_e) => 0,
        }
    }

    fn read_bytes(socket: &mut UdpSocket, buffer: &mut [u8; 65535]) -> Result<Vec<u8>, String> {
        let (amt, _src) = socket.recv_from(buffer).unwrap();
        let slice = &mut buffer[..amt];
        Ok(slice.to_vec())
    }
}
