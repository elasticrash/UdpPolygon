use std::net::{IpAddr, SocketAddr, UdpSocket};
pub mod config;
use config::Config;
use std::sync::mpsc::{self, Receiver, Sender};
use tokio;

#[derive(Debug)]
pub struct Polygon {
    pub socket: UdpSocket,
    pub buffer: [u8; 65535],
    pub destination: Option<SocketAddr>,
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
            .map(|addr| match addr.0 {
                IpAddr::V4(ipv4) => SocketAddr::new(IpAddr::V4(ipv4), addr.1),
                IpAddr::V6(ipv6) => SocketAddr::new(IpAddr::V6(ipv6), addr.1),
            })
            .collect::<Vec<_>>();

        let socket = UdpSocket::bind(&addrs[..]).unwrap();
        let buffer = [0_u8; 65535];
        Self {
            socket,
            buffer,
            destination: if let Some(addr) = config.destination_address {
                match addr.0 {
                    IpAddr::V4(ipv4) => Some(SocketAddr::new(IpAddr::V4(ipv4), addr.1)),
                    IpAddr::V6(ipv6) => Some(SocketAddr::new(IpAddr::V6(ipv6), addr.1)),
                }
            } else {
                None
            },
        }
    }
    pub fn receive(&mut self) -> Receiver<String> {
        let mut socket = self.socket.try_clone().unwrap();
        let mut buffer = self.buffer.clone();
        let (tx, rx) = Polygon::get_channel();
        tokio::spawn(async move {
            loop {
                let maybe: Option<String>;
                {
                    let packets_queued = peek(&mut socket, &mut buffer);
                    if packets_queued > 0 {
                        maybe = match read(&mut socket, &mut buffer) {
                            Ok(buf) => Some(buf),
                            Err(_) => None,
                        };

                        if let Some(data) = maybe {
                            tx.send(data).unwrap();
                        }
                    }
                }
            }
        });

        rx
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
