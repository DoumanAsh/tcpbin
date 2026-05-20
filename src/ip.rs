use crate::DEFAULT_LOCAL_IP;

use core::time;
use std::{io, net, thread};

use crate::utils::FmtBuffer;

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

    let mut buffer = FmtBuffer::new(&mut buffer);
    buffer.format_addr(remote_ip);
    if let Err(error) = socket.write_all(buffer.written_data()) {
        io_error!(error, "Write error", remote_ip);
        return;
    }
    if let Err(error) = socket.flush() {
        io_error!(error, "Flush error", remote_ip);
        return;
    }
    if let Err(error) = socket.shutdown(net::Shutdown::Both) {
        io_error!(error, "Shutdown error", remote_ip);
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

    #[cfg(feature = "tokio")]
    ///Returns future that binds socket to listen for incoming connections
    pub fn run_tokio(&self) -> impl core::future::Future<Output = Result<(), io::Error>> + Send + Sync + 'static {
        let addr = net::SocketAddr::new(self.addr, self.port);
        async move {
            let listener = tokio::net::TcpListener::bind(addr).await?;
            loop {
                let (mut socket, mut remote_ip) = listener.accept().await?;
                tokio::task::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};

                    let mut buffer_len = 0;
                    let mut buffer = [0u8; 128];
                    loop {
                        let read_op = tokio::time::timeout(READ_TIMEOUT, socket.read(&mut buffer[buffer_len..]));
                        match read_op.await {
                            Ok(Ok(0)) => break,
                            Ok(Ok(read_len)) => {
                                buffer_len += read_len
                            }
                            Ok(Err(error)) => {
                                error!("{}: Socket error(kind={:?}): {}", remote_ip, error.kind(), error);
                                return;
                            }
                            Err(_elapsed) => break,
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

                    let mut buffer = FmtBuffer::new(&mut buffer);
                    buffer.format_addr(remote_ip);
                    if let Err(error) = socket.write_all(buffer.written_data()).await {
                        io_error!(error, "Write error", remote_ip);
                        return;
                    }
                    if let Err(error) = socket.flush().await {
                        io_error!(error, "Flush error", remote_ip);
                        return;
                    }
                    if let Err(error) = socket.shutdown().await {
                        io_error!(error, "Shutdown error", remote_ip);
                    }
                });
            }
        }
    }
}
