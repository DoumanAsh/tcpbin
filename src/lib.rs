//!TCPBin - A simple utility TCP server

#![warn(missing_docs)]
#![allow(clippy::style)]

use core::net;

///Default local IP to be used
pub const DEFAULT_LOCAL_IP: net::IpAddr = net::IpAddr::V4(net::Ipv4Addr::UNSPECIFIED);

#[cfg(feature = "tracing")]
macro_rules! info {
    ($($input:tt)*) => {
        ::tracing::info!($($input)*)
    };
}

#[cfg(all(not(debug_assertions), not(feature = "tracing")))]
macro_rules! info {
    ($($input:tt)*) => {
        #[allow(unused)]
        let _ = (
            $($input)*
        );
    };
}

#[cfg(all(debug_assertions, not(feature = "tracing")))]
macro_rules! info {
    ($($input:tt)*) => {
        ::std::println!($($input)*)
    };
}

#[cfg(feature = "tracing")]
macro_rules! error {
    ($($input:tt)*) => {
        ::tracing::error!($($input)*)
    };
}

#[cfg(all(not(debug_assertions), not(feature = "tracing")))]
macro_rules! error {
    ($($input:tt)*) => {
        #[allow(unused)]
        let _ = (
            $($input)*
        );
    };
}

#[cfg(all(debug_assertions, not(feature = "tracing")))]
macro_rules! error {
    ($($input:tt)*) => {
        ::std::eprintln!($($input)*)
    };
}

macro_rules! io_error {
    ($error:expr, $description:literal, $addr:expr) => {
        use ::std::io::ErrorKind;
        let error = $error;
        if !::core::matches!(error.kind(), ErrorKind::BrokenPipe | ErrorKind::NotConnected) {
            error!("{}: {}(kind={:?}): {}", $addr, $description, error.kind(), error);
        } else {
            info!("{}: {}(kind={:?}): {}", $addr, $description, error.kind(), error);
        }
    };
}

mod utils;
mod ip;
pub use ip::IpEcho;
