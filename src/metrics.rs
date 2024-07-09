use serde::{Serialize};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RequestMetrics {
    pub host: String,
    pub response_time: u64,
    pub success: bool,
    pub timestamp: u64,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct LoadTestReport {
    pub metrics: Vec<RequestMetrics>,
    pub duration: u64,
    pub rps: u32,
    pub load_test_type: String,
}
