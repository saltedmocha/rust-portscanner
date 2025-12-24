use std::net::{self};

pub enum PortState {
    Open,
    Closed,
    Filtered,
}

pub struct NetInfo {
    pub address: net::IpAddr,
    pub port: Option<u16>,
    pub timeout: Option<u16>,
}
