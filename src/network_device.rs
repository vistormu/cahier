use std::net::IpAddr;

pub struct NetworkDevice {
    pub nickname: String,
    pub host: String,
    pub ip: IpAddr,
    pub port: u16,
}
