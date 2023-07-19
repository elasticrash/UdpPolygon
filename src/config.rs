use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bind_addresses: Vec<(IpAddr, u16)>,
    pub destination_address: (IpAddr, u16),
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_addresses: vec![(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5060)],
            destination_address: (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1020),
        }
    }
}

pub trait FromToml {
    fn from_toml(filename: String) -> Self;
}

pub trait FromArguments {
    fn from_arguments(local: Vec<(IpAddr, u16)>, remote: (IpAddr, u16)) -> Self;
}

pub trait FromDefault {
    fn from_default() -> Self;
}

pub trait FromEnv {
    fn from_env(envvar: String) -> Self;
}

impl FromToml for Config {
    fn from_toml(filename: String) -> Self {
        let contents = fs::read_to_string(filename);

        match contents {
            Ok(cf) => toml::from_str(&cf).unwrap(),
            Err(_) => Config::default(),
        }
    }
}

impl FromArguments for Config {
    fn from_arguments(local: Vec<(IpAddr, u16)>, remote: (IpAddr, u16)) -> Self {
        Config {
            bind_addresses: local,
            destination_address: remote,
        }
    }
}

impl FromDefault for Config {
    fn from_default() -> Self {
        Config::default()
    }
}

/// FromEnv only supports a single address/port pair.
/// This is because I wanted to avoid having to name
/// the enviroment variables, you can pass your own
/// instead
impl FromEnv for Config {
    fn from_env(envvar: String) -> Self {
        let address = match env::var(envvar).unwrap().parse::<IpAddr>() {
            Ok(addr) => addr,
            Err(err) => panic!("{}", err),
        };

        let port = match env::var("PORT").unwrap().parse::<u16>() {
            Ok(port) => port,
            Err(err) => panic!("{}", err),
        };

        Config {
            bind_addresses: vec![(address, port)],
            destination_address: (address, port),
        }
    }
}
