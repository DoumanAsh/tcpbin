//!TCPBin - A simple utility TCP server

#![warn(missing_docs)]
#![allow(clippy::style)]

use core::net;

///Default local IP to be used
pub const DEFAULT_LOCAL_IP: net::IpAddr = net::IpAddr::V4(net::Ipv4Addr::UNSPECIFIED);

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

mod ip;
pub use ip::IpEcho;
