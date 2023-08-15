use std::net::{IpAddr, SocketAddr, UdpSocket};
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

#[derive(Debug)]
pub struct Polygon {
    pub socket: UdpSocket,
    pub buffer: [u8; 65535],
    pub destination: Option<SocketAddr>,
    pub counter: Arc<Mutex<u32>>,
}

impl Polygon {
    pub fn get_channel() -> (Sender<String>, Receiver<String>) {
        let (tx, rx) = mpsc::channel();
        (tx, rx)
    }
    pub fn configure(config: Config) -> Self {
        let addrs = config
            .bind_addresses
            .to_vec()
            .into_iter()
            .map(|addr| match addr.ip {
                IpAddr::V4(ipv4) => SocketAddr::new(IpAddr::V4(ipv4), addr.port),
                IpAddr::V6(ipv6) => SocketAddr::new(IpAddr::V6(ipv6), addr.port),
            })
            .collect::<Vec<_>>();

        let socket = UdpSocket::bind(&addrs[..]).unwrap();
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
            counter: Arc::new(Mutex::new(0)),
        }
    }
    pub fn receive(&mut self) -> Receiver<String> {
        let mut socket = self.socket.try_clone().unwrap();
        let mut buffer = self.buffer.clone();
        let counter = Arc::clone(&self.counter);
        let (tx, rx) = Polygon::get_channel();
        tokio::spawn(async move {
            loop {
                let maybe: Option<String>;
                {
                    let packets_queued = UdpRead::peek(&mut socket, &mut buffer);
                    if packets_queued > 0 {
                        maybe = match UdpRead::read(&mut socket, &mut buffer) {
                            Ok(buf) => Some(buf),
                            Err(_) => None,
                        };

                        if let Some(data) = maybe {
                            *counter.lock().unwrap() += 1;

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
    pub fn send_with_timer(&mut self, data: String, timers: Timers) {
        let socket = self.socket.try_clone().unwrap();
        let destination = self.destination.clone().unwrap();
        let counter = Arc::clone(&self.counter);
        tokio::spawn(async move {
            let receive_counter = *counter.lock().unwrap();
            let mut current_timer = timers.intervals.into_iter();
            loop {
                if receive_counter != *counter.lock().unwrap() {
                    *counter.lock().unwrap() = 0;
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
                        data.as_bytes(),
                        format!("{}:{}", &destination.ip(), &destination.port()),
                    )
                    .unwrap();
                tokio::time::sleep(Duration::from_millis(next_timer)).await;
            }
        });
    }
    pub fn send(&mut self, data: String) {
        let destination = self.destination.clone().unwrap();
        self.socket
            .send_to(
                data.as_bytes(),
                format!("{}:{}", &destination.ip(), &destination.port()),
            )
            .unwrap();
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

    fn read(socket: &mut UdpSocket, buffer: &mut [u8; 65535]) -> Result<String, String> {
        let (amt, _src) = socket.recv_from(buffer).unwrap();
        let slice = &mut buffer[..amt];
        slice.to_vec();
        let message = String::from_utf8_lossy(slice);
        Ok(message.to_string())
    }
}
