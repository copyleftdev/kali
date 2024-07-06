use std::net::TcpStream;
use std::io::{Write, Read};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::metrics::{RequestMetrics, LoadTestReport};
use crate::config::Config;

pub fn perform_load_test(config: &Config) -> LoadTestReport {
    let end_time = Instant::now() + Duration::from_secs(config.duration);
    let requests = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for _ in 0..config.rps {
        let requests = Arc::clone(&requests);
        let host = config.host.clone();
        let port = config.port;
        let rps = config.rps;
        let payload = config.payload.clone();

        let handle = thread::spawn(move || {
            while Instant::now() < end_time {
                let start_time = Instant::now();
                let success = match TcpStream::connect((&host as &str, port)) {
                    Ok(mut stream) => {
                        stream.write_all(payload.as_bytes()).unwrap();
                        let mut buffer = [0; 1024];
                        let _ = stream.read(&mut buffer).unwrap();
                        true
                    }
                    Err(_) => false,
                };
                let response_time = start_time.elapsed().as_micros() as u64;

                let mut requests = requests.lock().unwrap();
                requests.push(RequestMetrics {
                    response_time,
                    success,
                    timestamp: start_time.elapsed().as_secs(),
                });

                thread::sleep(Duration::from_millis(1000 / rps as u64));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let requests = Arc::try_unwrap(requests).unwrap().into_inner().unwrap();

    LoadTestReport {
        host: config.host.clone(),
        port: config.port,
        duration: config.duration,
        rps: config.rps,
        load_test_type: config.load_test_type.clone(),
        requests,
    }
}
