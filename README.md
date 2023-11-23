# UDP Polygon

An opiniated UDP listener and publisher

## Breaking Changes
From 0.1.1 to 0.2.0
Previously, versions below 0.1.1 were converting the received datagram bytes into a string. However, starting from version 0.2.0 and onwards, the receive event now delivers the bytes directly.
Additionally, the expected format for sending data now requires it to be encapsulated within a Vec<u8> for compatibility.
The change was implemented because the previous implementation was found to be too restrictive.

## Requirements

* the consumer requires tokio
* a producer does not require anything extra
* a producer with the timer flag enabled requires tokio

## Configuration

There are many options on configuring your UDP client and server

* TOML file
``` Toml
[[bind_addresses]]
ip = "127.0.0.1"
port = 5061
[destination_address]
ip = "127.0.0.1"
port = 5060
```
* Arguments

``` rust
 let config = Config::from_arguments(
        vec![(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5061)],
        Some((IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5060)),
    );
```

* Enviroment variables

``` bash 
export BIND_ADDRS=127.0.0.1
export BIND_PORT=5061
export DEST_ADDRS=127.0.0.1
export DEST_PORT=5060
```

``` rust
 let config = Config::from_env();
```

## Send

``` rust 
    polygon.send("Hello World".as_bytes().to_vec());
``` 

## Receive

``` rust
    let rx = polygon.receive();
 
    loop {
        let maybe = rx.try_recv();
        if let Ok(data) = maybe {
            println!("receiving... {data:?}");
        }
    }
```

## Basic Examples

* send_fa (example by passing arguments)
* receive_fa (example by passing arguments)
* send_toml (example by using a toml file)
* receive_toml (example by using a toml file)
* send_receive (both sending and receiving)

## Timer flag

Retransmits a message with specific delays 

``` rust
  polygon.send_with_timer(
        "Hello World".as_bytes().to_vec(),
        Timers {
            delays: vec![500, 600, 1000, 1500],
        },
    );

```

retransmissions can be paused at any given time, even mid sending a message, effectively cancelling a retransmission

``` rust
    let mut polygon = Polygon::configure(config);
    let pause = Arc::clone(&polygon.pause_timer_send);
    *pause.lock().unwrap() = true;
```
or 
``` rust
    let mut polygon = Polygon::configure(config);
    polygon.pause_timer_send()
    polygon.resume_timer_send()
```
this will make the send_with_timer to behave like a normal send (only it would still require tokio)

## Timer Examples
* send_receive_with_timer






