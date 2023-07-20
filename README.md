# UDP Polygon

An opiniated UDP listener and publisher

## Configuration

There are many ways to configure your UDP client and server

* From a TOML file
* By passing arguments

``` Rust
 let config = Config::from_arguments(
        vec![(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5061)],
        Some((IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5060)),
    );
```

* From enviroment variables

``` bash 
export BIND_ADDRS=127.0.0.1
export BIND_PORT=5061
export DEST_ADDRS=127.0.0.1
export DEST_PORT=5060
```

``` rust
 let config = Config::from_env();
```

## Examples

At the moment there are only examples by passing arguments

More examples comming soon
