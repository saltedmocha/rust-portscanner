use std::process::exit;

use clap::Parser;
use rust_portscanner::arg;

fn main() {
    let a = arg::GlobalArg::parse();
    match a.command {
        arg::Commands::Full(_arg) => {}
        arg::Commands::Null(_arg) => {}
        arg::Commands::Ping(_arg) => {}
        arg::Commands::Quick(_arg) => {}
        arg::Commands::Udp(_arg) => {}
        arg::Commands::Xmas(_arg) => {}
    }
    exit(0)
}
