use clap::{Parser, Subcommand};
use std::{net, process::exit};

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    /// Use ping scanning method
    Ping,
    #[command(arg_required_else_help = true)]
    /// Use TCP Half Open method
    Quick,
    #[command(arg_required_else_help = true)]
    /// Use TCP Connect method
    Full,
    #[command(arg_required_else_help = true)]
    /// Use udp method, best for target such as dns req
    Udp,
    #[command(arg_required_else_help = true)]
    /// Use XMAS scan / NULL packet method
    Stealth,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GlobalArg {
    #[arg(short = '4', long)]
    /// Use IPv4 [default]
    v4: bool,
    #[arg(short = '6', long)]
    /// Use IPv6
    v6: bool,
    #[arg(short, long, default_value = "127.0.0.1")]
    /// Default to scan user machine instead
    target: String,

    #[command(subcommand)]
    command: Commands,
}

impl GlobalArg {
    pub fn parse_ip(self) -> net::IpAddr {
        if self.v6 {
            let address: net::Ipv6Addr = match self.target.parse::<net::Ipv6Addr>() {
                Ok(ip) => ip,
                Err(err) => {
                    println!(
                        "Failed to parse IP address, please enter a valid address\nerr: {}",
                        err
                    );
                    exit(1);
                }
            };
            return net::IpAddr::V6(address);
        }

        let address: net::Ipv4Addr = match self.target.parse::<net::Ipv4Addr>() {
            Ok(ip) => ip,
            Err(err) => {
                println!(
                    "Failed to parse IP address, please enter a valid address\nerr: {}",
                    err
                );
                exit(1);
            }
        };
        net::IpAddr::V4(address)
    }
}
