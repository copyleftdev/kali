use serde::{Serialize};

#[derive(Serialize, Debug, PartialEq)]
pub struct RequestMetrics {
    pub response_time: u64,
    pub success: bool,
    pub timestamp: u64,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct LoadTestReport {
    pub host: String,
    pub port: u16,
    pub duration: u64,
    pub rps: u32,
    pub load_test_type: String,
    pub requests: Vec<RequestMetrics>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_request_metrics_serialization() {
        let metrics = RequestMetrics {
            response_time: 100,
            success: true,
            timestamp: 1622549763,
        };
        let json = serde_json::to_string(&metrics).unwrap();
        let expected_json = r#"{"response_time":100,"success":true,"timestamp":1622549763}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_load_test_report_serialization() {
        let report = LoadTestReport {
            host: "127.0.0.1".to_string(),
            port: 8080,
            duration: 10,
            rps: 100,
            load_test_type: "tcp".to_string(),
            requests: vec![
                RequestMetrics {
                    response_time: 100,
                    success: true,
                    timestamp: 1622549763,
                },
                RequestMetrics {
                    response_time: 200,
                    success: false,
                    timestamp: 1622549764,
                },
            ],
        };
        let json = serde_json::to_string(&report).unwrap();
        let expected_json = r#"{"host":"127.0.0.1","port":8080,"duration":10,"rps":100,"load_test_type":"tcp","requests":[{"response_time":100,"success":true,"timestamp":1622549763},{"response_time":200,"success":false,"timestamp":1622549764}]}"#;
        assert_eq!(json, expected_json);
    }
}
