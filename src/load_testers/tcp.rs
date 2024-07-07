use std::net::TcpStream;
use std::io::{Write, Read};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;
use crate::metrics::{RequestMetrics, LoadTestReport};
use crate::config::Config;

pub fn perform_load_test(config: &Config) -> LoadTestReport {
    let end_time = Instant::now() + Duration::from_secs(config.duration);
    let requests = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    let duration_per_request = Duration::from_millis(1000 / config.rps as u64);

    for _ in 0..config.rps {
        let requests = Arc::clone(&requests);
        let host = config.host.clone();
        let port = config.port;
        let payload = config.payload.clone();
        let jitter = config.jitter;

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while Instant::now() < end_time {
                let start_time = Instant::now();
                let success = match TcpStream::connect((&host as &str, port)) {
                    Ok(mut stream) => {
                        if stream.write_all(payload.as_bytes()).is_ok() {
                            let mut buffer = [0; 1024];
                            if stream.read(&mut buffer).is_ok() {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                };
                let response_time = start_time.elapsed().as_micros() as u64;

                // Capture the current system time as a UNIX timestamp
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                let mut requests = requests.lock().unwrap();
                requests.push(RequestMetrics {
                    response_time,
                    success,
                    timestamp,
                });

                let jitter_value = rng.gen_range(0..jitter);
                let sleep_duration = duration_per_request + Duration::from_millis(jitter_value);
                let elapsed = start_time.elapsed();

                if sleep_duration > elapsed {
                    thread::sleep(sleep_duration - elapsed);
                }
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
