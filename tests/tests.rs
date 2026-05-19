use std::{net, thread, time};
use std::io::{self, Write};

use tcpbin::{DEFAULT_LOCAL_IP, IpEcho};

#[test]
fn should_detect_ip() {
    const PORT: u16 = 45001;
    const ADDR: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::LOCALHOST), PORT);

    let _ = thread::spawn(|| {
        let _ = IpEcho::new().with_port(PORT).run_blocking();
    });

    std::thread::sleep(time::Duration::from_secs(1));
    let mut socket = net::TcpStream::connect(ADDR).expect("to connect");
    std::thread::yield_now();

    let mut out = String::new();
    io::Read::read_to_string(&mut socket, &mut out).expect("to read ip");
    assert_eq!(out, "127.0.0.1");
}

#[test]
fn should_detect_ip_via_proxy_v1() {
    const PORT: u16 = 45002;
    const ADDR: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::LOCALHOST), PORT);

    let proxy = ha_proxy_protocol::v1::Proxy {
        src: net::SocketAddr::new(net::Ipv4Addr::new(255, 1, 2, 3).into(), 80),
        dst: net::SocketAddr::new(DEFAULT_LOCAL_IP, PORT),
    };
    let _ = thread::spawn(|| {
        let _ = IpEcho::new().with_port(PORT).run_blocking();
    });

    std::thread::sleep(time::Duration::from_secs(1));
    let mut socket = net::TcpStream::connect(ADDR).expect("to connect");
    socket.write_fmt(format_args!("{proxy}\r\n")).expect("write proxy");
    socket.flush().expect("flush proxy");
    std::thread::yield_now();

    let mut out = String::new();
    io::Read::read_to_string(&mut socket, &mut out).expect("to read ip");
    assert_eq!(out, "255.1.2.3");
}
