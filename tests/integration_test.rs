use kali::config::Config;
use kali::load_testers::tcp::perform_load_test;

#[test]
fn test_tcp_load_test() {
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 8080,
        duration: 5,
        rps: 10,
        load_test_type: "tcp".to_string(),
        output_file: "output.json".to_string(),
        payload: "Hello World".to_string(),
        jitter: 50,
        help: None,
    };

    let report = perform_load_test(&config);
    assert_eq!(report.host, "127.0.0.1");
    assert_eq!(report.port, 8080);
    assert_eq!(report.duration, 5);
    assert_eq!(report.rps, 10);
    assert_eq!(report.load_test_type, "tcp");
    assert!(report.requests.len() > 0);
    for request in report.requests {
        assert!(request.response_time > 0);
        assert!(request.timestamp > 0);
    }
}
