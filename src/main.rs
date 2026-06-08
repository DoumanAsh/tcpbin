#![allow(clippy::style)]

use arg::Args;

use core::net::{IpAddr, Ipv4Addr};

#[derive(Args, Debug)]
#[arg(infer_name)]
///TCPBin server
struct Cli {
    #[arg(long = "host", default_value = "IpAddr::V4(Ipv4Addr::UNSPECIFIED)")]
    ///Specifies IP address to bind server with. Defaults to 0.0.0.0
    host: IpAddr,
    #[arg(long = "echo-port", default_value = "59000")]
    ///Specifies port for Data Echo server. Defaults to 59000.
    echo_port: u16,
    #[arg(long = "ip-echo-port", default_value = "59001")]
    ///Specifies port for IP Echo server. Defaults to 59001.
    ip_echo_port: u16,
}

#[cfg(feature = "tokio")]
fn run(args: Cli) {
    let rt = match tokio::runtime::Builder::new_current_thread().enable_io().enable_time().name("tokio-thread").build_local(Default::default()) {
        Ok(rt) => rt,
        Err(error) => {
            tcpbin::error!("Failed to create IO runtime: {}", error);
            return;
        }
    };

    let ip_echo = tcpbin::IpEcho::new().with_ip(args.host).with_port(args.ip_echo_port);
    let data_echo = tcpbin::Echo::new().with_ip(args.host).with_port(args.echo_port);
    rt.block_on(async {
        tcpbin::info!("Data Echo listening {}", data_echo.addr());
        tcpbin::info!("IP Echo listening {}", ip_echo.addr());
        tokio::select!{
            data_echo = data_echo.run_tokio() => {
                if let Err(error) = data_echo {
                    tcpbin::error!("Failed to start Echo server: {}", error);
                }
            }

            ip_echo = ip_echo.run_tokio() => {
                if let Err(error) = ip_echo {
                    tcpbin::error!("Failed to start IpEcho server: {}", error);
                }
            }
        };
    })
}

#[cfg(not(feature = "tokio"))]
fn run(args: Cli) {
    let data_echo = tcpbin::IpEcho::new().with_ip(args.host).with_port(args.echo_port);
    let ip_echo = tcpbin::Echo::new().with_ip(args.host).with_port(args.ip_echo_port);

    std::thread::scope(|scope| {
        scope.spawn(move || {
            tcpbin::info!("Data Echo listening {}", data_echo.addr());
            if let Err(error) = data_echo.run_blocking() {
                tcpbin::error!("Failed to start IpEcho server: {}", error);
            }
        });

        scope.spawn(move || {
            tcpbin::info!("IP Echo listening {}", ip_echo.addr());
            if let Err(error) = ip_echo.run_blocking() {
                tcpbin::error!("Failed to start IpEcho server: {}", error);
            }
        });
    })
}

fn main() {
    run(arg::parse_args());
}
