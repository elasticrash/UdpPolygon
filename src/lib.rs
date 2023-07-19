use std::net::{IpAddr, SocketAddr, UdpSocket};
pub mod config;
use config::Config;

#[derive(Debug)]
pub struct Polygon {
    pub id: u32,
    pub msg: String,
}

pub fn configure(config: Config) -> (UdpSocket, [u8; 65535]) {
    let addrs = config
        .bind_addresses
        .into_iter()
        .map(|addr| match addr.0 {
            IpAddr::V4(ipv4) => SocketAddr::new(IpAddr::V4(ipv4), addr.1),
            IpAddr::V6(ipv6) => SocketAddr::new(IpAddr::V6(ipv6), addr.1),
        })
        .collect::<Vec<_>>();

    // tokio::spawn(async move {
    let mut socket = UdpSocket::bind(&addrs[..]).unwrap();
    let mut buffer = [0_u8; 65535];

    //   consume(&mut socket, &mut buffer);
    //});

    (socket, buffer)
}

pub fn consume(socket: &mut UdpSocket, buffer: &mut [u8]) {
    loop {
        let mut maybe: Option<String> = None;
        {
            let packets_queued = peek(socket, buffer);
            println!("packets_queued {packets_queued}");
            if packets_queued > 0 {
                maybe = match receive(socket, buffer) {
                    Ok(buf) => Some(buf),
                    Err(_) => None,
                };

                if let Some(data) = maybe {
                    println!("data {data:?}");
                }
            }
        }
    }
}

pub fn send(socket: &mut UdpSocket, destination: &SocketAddr, data: &Polygon) {
    println!("data {data:?}");
    socket
        .send_to(
            &data.msg.as_bytes(),
            format!("{}:{}", &destination.ip(), &destination.port()),
        )
        .unwrap();
}

pub fn receive(socket: &mut UdpSocket, buffer: &mut [u8]) -> Result<String, ()> {
    let (amt, _src) = socket.recv_from(buffer).unwrap();
    let slice = &mut buffer[..amt];
    slice.to_vec();
    let message = String::from_utf8_lossy(&slice);
    Ok(message.to_string())
}

pub fn peek(socket: &mut UdpSocket, buffer: &mut [u8]) -> usize {
    match socket.peek(buffer) {
        Ok(received) => received,
        Err(_e) => 0,
    }
}
