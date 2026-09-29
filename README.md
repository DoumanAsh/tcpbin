# TCPBin

[![Rust](https://github.com/DoumanAsh/tcpbin/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/tcpbin/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/tcpbin.svg)](https://crates.io/crates/tcpbin)
[![Documentation](https://docs.rs/tcpbin/badge.svg)](https://docs.rs/crate/tcpbin/)
[![Hub](https://img.shields.io/badge/Docker-Hub-2496ed.svg)](https://hub.docker.com/r/douman/tcpbin/tags)
[![Quay](https://img.shields.io/badge/quay.io-douman%2Ftcpbin-blue)](https://quay.io/repository/doumanash/tcpbin)

TCPBin - A simple utility TCP server

MSRV 1.85

## Build features

- `cli` - Enables to build command line binary to run server
- `tokio` - Enables async version of all server handlers. Otherwise there is simple blocking version
- `tracing` - Enables `tracing` logging

## Usage

You can download pre-built binaries [here](https://github.com/DoumanAsh/tcpbin/releases/latest)

```
tcpbin 0.1.3
TCPBin server

USAGE: [OPTIONS]

OPTIONS:
    -h,  --help                         Prints this help information
         --host <host>                  Specifies IP address to bind server with. Defaults to 0.0.0.0. Can be set via env TCPBIN_HOST
         --echo-port <echo_port>        Specifies port for Data Echo server. Defaults to 59000. Can be set via env TCPBIN_ECHO_PORT
         --ip-echo-port <ip_echo_port>  Specifies port for IP Echo server. Defaults to 59001. Can be set via env TCPBIN_IP_ECHO_PORT
```

## Testing server

- `roseline.servebeer.com:59000` - Echo endpoint, returns back all data
- `roseline.servebeer.com:59001` - IP Echo endpoint, returns client's IP on connect. Can be over-written using [proxy protocol](https://www.haproxy.org/download/1.8/doc/proxy-protocol.txt)
