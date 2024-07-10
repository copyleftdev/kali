use kali::config::Config;
use kali::load_testers::tcp::perform_load_test;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_single_host_load_test() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap(); // Changed port to 8081

    tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => return, // connection closed
                        Ok(n) => {
                            // echo the received data back to the client
                            if socket.write_all(&buf[..n]).await.is_err() {
                                return; // connection closed
                            }
                        }
                        Err(_) => return,
                    }
                }
            });
        }
    });

    let config = Config {
        host: Some("127.0.0.1".to_string()),
        hosts_and_biases: None,
        port: 8081, // Updated port
        duration: 5,
        rps: 10,
        load_test_type: "tcp".to_string(),
        output_file: "output_single.json".to_string(),
        payload: "Hello World".to_string(),
        jitter: 50,
        help: None,
    };

    let report = perform_load_test(&config).await;

    assert_eq!(report.duration, 5);
    assert_eq!(report.rps, 10);
    assert_eq!(report.load_test_type, "tcp");
    assert!(report.metrics.len() > 0);

    for metric in report.metrics {
        assert_eq!(metric.host, "127.0.0.1");
        assert!(metric.response_time > 0);
        assert!(metric.timestamp > 0);
    }
}
