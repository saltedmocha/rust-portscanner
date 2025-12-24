use std::{net, process};

use clap::{Parser, Subcommand};

pub enum AddressMode {
    V4,
    V6,
}

#[derive(Parser, Debug)]
pub struct Arg {
    #[arg(short = '4', long)]
    /// Use IPv4
    v4: bool,
    #[arg(short = '6', long)]
    /// Use IPv6
    v6: bool,
    #[arg(short, long)]
    /// Check for specific port
    port: Option<u16>,
    #[arg(short, long)]
    /// Set time until timeout
    timeout: Option<u16>,

    /// Target or destination IP address, accept IPv4 or IPv6 unless specified
    target: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// Use TCP Connect method
    Full(Arg),
    #[command(arg_required_else_help = true)]
    /// Use Null packet method
    Null(Arg),
    #[command(arg_required_else_help = true)]
    /// Use TCP Half Open method
    Quick(Arg),
    #[command(arg_required_else_help = true)]
    /// Use ping scanning method
    Ping(Arg),
    #[command(arg_required_else_help = true)]
    /// Use udp scan method
    Udp(Arg),
    #[command(arg_required_else_help = true)]
    /// Use xmas scan method
    Xmas(Arg),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GlobalArg {
    #[command(subcommand)]
    pub command: Commands,
}

impl Arg {
    pub fn mode(&self) -> AddressMode {
        if self.v6 {
            return AddressMode::V6;
        }

        AddressMode::V4
    }

    pub fn port(&self) -> &Option<u16> {
        &self.port
    }

    pub fn parse_ip(&self) -> net::IpAddr {
        if self.v6 {
            let address: Result<net::Ipv6Addr, net::AddrParseError> =
                self.target.parse::<net::Ipv6Addr>();
            match address {
                Ok(ip) => return net::IpAddr::V6(ip),
                Err(_) => {
                    println!("Invalid IPv6 address, please makesure address fit the IPv6 format");
                    process::exit(1);
                }
            }
        }

        let address: Result<net::Ipv4Addr, net::AddrParseError> =
            self.target.parse::<net::Ipv4Addr>();
        match address {
            Ok(ip) => net::IpAddr::V4(ip),
            Err(_) => {
                println!("Invalid IPv4 address, plelase makesure address fit the IPv4 format");
                process::exit(1);
            }
        }
    }

    pub fn timeout(&self) -> &Option<u16> {
        &self.timeout
    }
}
