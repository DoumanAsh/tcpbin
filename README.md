# TCPBin

[![Rust](https://github.com/DoumanAsh/tcpbin/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/tcpbin/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/tcpbin.svg)](https://crates.io/crates/tcpbin)
[![Documentation](https://docs.rs/tcpbin/badge.svg)](https://docs.rs/crate/tcpbin/)

TCPBin - A simple utility TCP server

MSRV 1.85

## Build features

- `cli` - Enables to build command line binary to run server
- `tokio` - Enables async version of all server handlers. Otherwise there is simple blocking version
- `tracing` - Enables `tracing` logging

## Usage

You can download pre-built binaries [here](https://github.com/DoumanAsh/tcpbin/releases/latest)

```
tcpbin 0.1.2
TCPBin server

USAGE: [OPTIONS]

OPTIONS:
    -h,  --help                         Prints this help information
         --host <host>                  Specifies IP address to bind server with. Defaults to 0.0.0.0
         --echo-port <echo_port>        Specifies port for Data Echo server. Defaults to 59000.
         --ip-echo-port <ip_echo_port>  Specifies port for IP Echo server. Defaults to 59001.
```

## Testing server

- `roseline.servebeer.com:59000` - Echo endpoint, returns back all data
- `roseline.servebeer.com:59001` - IP Echo endpoint, returns client's IP on connect. Can be over-written using [proxy protocol](https://www.haproxy.org/download/1.8/doc/proxy-protocol.txt)
