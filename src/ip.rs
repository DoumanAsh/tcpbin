use crate::DEFAULT_LOCAL_IP;

use core::time;
use std::{io, net, thread};

//If we receive haproxy info, it will come immediately upon connection establishment
//otherwise we do not care for user's input
const READ_TIMEOUT: time::Duration = time::Duration::from_secs(1);

fn blocking_socket_handler(mut socket: net::TcpStream, mut remote_ip: net::SocketAddr) {
    use io::{Read, Write};

    let mut buffer_len = 0;
    let mut buffer = [0u8; 128];
    if let Err(error) = socket.set_read_timeout(Some(READ_TIMEOUT)) {
        error!("{}: Socket timeout set error: {}", remote_ip, error);
        return;
    }

    loop {
        match socket.read(&mut buffer[buffer_len..]) {
            Ok(0) => break,
            Ok(read_len) => {
                buffer_len += read_len
            }
            Err(error) if error.kind() == io::ErrorKind::TimedOut || error.kind() == io::ErrorKind::WouldBlock => break,
            Err(error) => {
                error!("{}: Socket error(kind={:?}): {}", remote_ip, error.kind(), error);
                return;
            }
        }
    }

    if let Ok(proxy_info) = ha_proxy_protocol::parse(&buffer[..buffer_len]) {
        let (proxy_info, _) = proxy_info.into_generic();
        if let Some(proxy_info) = proxy_info {
            if let ha_proxy_protocol::Addr::Inet(real_addr) = proxy_info.src {
                remote_ip = real_addr;
            }
        }
    }

    if let Err(error) = socket.write_fmt(format_args!("{}", remote_ip.ip())) {
        error!("{}: Write error: {}", remote_ip, error);
    }
    if let Err(error) = socket.flush() {
        error!("{}: Flush error: {}", remote_ip, error);
    }
    if let Err(error) = socket.shutdown(net::Shutdown::Both) {
        error!("{}: Shutdown error: {}", remote_ip, error);
    }
}

#[derive(Copy, Clone)]
///Server that returns client's IP and closes connection
pub struct IpEcho {
    addr: net::IpAddr,
    port: u16
}

impl IpEcho {
    #[inline]
    ///Creates new server with default port 59001
    pub const fn new() -> Self {
        Self {
            addr: DEFAULT_LOCAL_IP,
            port: 59001,
        }
    }

    #[inline]
    ///Changes the ip to be used to bind socket
    pub const fn with_ip(mut self, addr: net::IpAddr) -> Self {
        self.addr = addr;
        self
    }

    #[inline]
    ///Changes the port to be used to bind socket
    pub const fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    ///Binds socket, starting to accept connections continuously
    ///
    ///This never exits unless underlying OS error causes it to stop accepting new connection
    pub fn run_blocking(&self) -> Result<(), io::Error> {
        let addr = net::SocketAddr::new(self.addr, self.port);
        let listener = net::TcpListener::bind(addr)?;

        loop {
            let (socket, remote_addr) = listener.accept()?;

            if let Err(error) = thread::Builder::new().name("tcp-connection-handler".to_owned()).spawn(move || blocking_socket_handler(socket, remote_addr)) {
                return Err(io::Error::other(error));
            }
        }
    }
}
