use serde::{Serialize, Deserialize}; // Added Deserialize

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)] // Added Deserialize and Clone traits
pub struct RequestMetrics {
    pub host: String,
    pub response_time: u64,
    pub success: bool,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)] // Added Deserialize and Clone traits
pub struct LoadTestReport {
    pub metrics: Vec<RequestMetrics>,
    pub duration: u64,
    pub rps: u32,
    pub load_test_type: String,
}
