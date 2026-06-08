use std::{io, net, thread, time};

use crate::{DEFAULT_LOCAL_IP, READ_TIMEOUT};

const BUFFER_SIZE: usize = 256;
const WRITE_TIMEOUT: time::Duration = time::Duration::from_secs(10);

fn blocking_socket_handler(mut socket: net::TcpStream) {
    use io::{Read, Write};

    let mut buffer = [0u8; BUFFER_SIZE];
    if let Err(error) = socket.set_read_timeout(Some(READ_TIMEOUT)) {
        error!("Socket read timeout set error: {}", error);
        return;
    }
    if let Err(error) = socket.set_write_timeout(Some(WRITE_TIMEOUT)) {
        error!("Socket write timeout set error: {}", error);
        return;
    }

    'read: loop {
        match socket.read(&mut buffer) {
            Ok(0) => break,
            Ok(read_len) => 'write: loop {
                match socket.write_all(&mut buffer[..read_len]) {
                    Ok(()) => {
                        if let Err(error) = socket.flush() {
                            io_error!(error, "Flush error");
                            return;
                        }
                        continue 'read;
                    },
                    Err(error) if error.kind() == io::ErrorKind::TimedOut || error.kind() == io::ErrorKind::WouldBlock => break 'read,
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => continue 'write,
                    Err(error) => {
                        error!("Socket write error(kind={:?}): {}", error.kind(), error);
                        return;
                    }
                }
            }
            Err(error) if error.kind() == io::ErrorKind::TimedOut || error.kind() == io::ErrorKind::WouldBlock => break,
            Err(error) => {
                error!("Socket read error(kind={:?}): {}", error.kind(), error);
                return;
            }
        }
    }

    if let Err(error) = socket.flush() {
        io_error!(error, "Flush error");
        return;
    }
    if let Err(error) = socket.shutdown(net::Shutdown::Both) {
        io_error!(error, "Shutdown error");
    }
}

#[derive(Copy, Clone)]
///Server that returns client's data back to it
pub struct Echo {
    addr: net::IpAddr,
    port: u16
}

impl Echo {
    #[inline]
    ///Creates new server with default port 59000
    pub const fn new() -> Self {
        Self {
            addr: DEFAULT_LOCAL_IP,
            port: 59000,
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

    #[inline]
    ///Returns address to be used
    pub const fn addr(&self) -> net::SocketAddr {
        net::SocketAddr::new(self.addr, self.port)
    }

    ///Binds socket, starting to accept connections continuously
    ///
    ///This never exits unless underlying OS error causes it to stop accepting new connection
    pub fn run_blocking(&self) -> Result<(), io::Error> {
        let addr = self.addr();
        let listener = net::TcpListener::bind(addr)?;

        loop {
            let (socket, _) = listener.accept()?;

            if let Err(error) = thread::Builder::new().name("tcp-connection-handler".to_owned()).spawn(move || blocking_socket_handler(socket)) {
                return Err(io::Error::other(error));
            }
        }
    }

    #[cfg(feature = "tokio")]
    ///Returns future that binds socket to listen for incoming connections
    pub fn run_tokio(&self) -> impl core::future::Future<Output = Result<(), io::Error>> + Send + Sync + 'static {
        let addr = self.addr();
        async move {
            let listener = tokio::net::TcpListener::bind(addr).await?;
            loop {
                let (mut socket, _) = listener.accept().await?;
                tokio::task::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};

                    let mut buffer = [0u8; BUFFER_SIZE];
                    'read: loop {
                        let read_op = tokio::time::timeout(READ_TIMEOUT, socket.read(&mut buffer));
                        match read_op.await {
                            Ok(Ok(0)) => break 'read,
                            Ok(Ok(read_len)) => match tokio::time::timeout(WRITE_TIMEOUT, socket.write_all(&buffer[..read_len])).await {
                                Ok(Ok(())) => {
                                    if let Err(error) = socket.flush().await {
                                        io_error!(error, "Flush error");
                                        return;
                                    }
                                    continue 'read;
                                },
                                Ok(Err(error)) => {
                                    error!("Socket write error(kind={:?}): {}", error.kind(), error);
                                    return;
                                }
                                Err(_elapsed) => break 'read,
                            },
                            Ok(Err(error)) => {
                                error!("Socket read error(kind={:?}): {}", error.kind(), error);
                                return;
                            }
                            Err(_elapsed) => break 'read,
                        }
                    }

                    if let Err(error) = socket.flush().await {
                        io_error!(error, "Flush error");
                        return;
                    }
                    if let Err(error) = socket.shutdown().await {
                        io_error!(error, "Shutdown error");
                    }
                });
            }
        }
    }
}
