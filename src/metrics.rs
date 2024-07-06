use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct RequestMetrics {
    pub response_time: u64,
    pub success: bool,
    pub timestamp: u64,
}

#[derive(Serialize, Debug)]
pub struct LoadTestReport {
    pub host: String,
    pub port: u16,
    pub duration: u64,
    pub rps: u32,
    pub load_test_type: String,
    pub requests: Vec<RequestMetrics>,
}
