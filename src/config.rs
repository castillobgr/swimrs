
use std::net::IpAddr;

/// Configuration for a Guestlist instance.
pub struct Config {
    // The address to bind on
    pub address: IpAddr,
    // The port to listen on
    pub port: String,
    // The time to wait between failure detection "rounds"
    pub detection_period_ms: f64,
    //
    pub detection_ping_timeout: f64,
    // The number of random cluster members to contact each
    pub detection_group_size: u16,
}