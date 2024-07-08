use std::net::TcpStream;
use std::io::{Write, Read};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use rand::Rng;
use crate::metrics::{RequestMetrics, LoadTestReport};
use crate::config::Config;

pub fn perform_load_test(config: &Config) -> LoadTestReport {
    let end_time = Instant::now() + Duration::from_secs(config.duration);
    let requests = Arc::new(Mutex::new(Vec::new()));
    let success_count = Arc::new(AtomicUsize::new(0));
    let fail_count = Arc::new(AtomicUsize::new(0));
    let total_jitter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let duration_per_request = Duration::from_millis(1000 / config.rps as u64);

    for _ in 0..config.rps {
        let requests = Arc::clone(&requests);
        let success_count = Arc::clone(&success_count);
        let fail_count = Arc::clone(&fail_count);
        let total_jitter = Arc::clone(&total_jitter);
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

                // Update counters
                if success {
                    success_count.fetch_add(1, Ordering::SeqCst);
                } else {
                    fail_count.fetch_add(1, Ordering::SeqCst);
                }

                // Calculate jitter
                let jitter_value = rng.gen_range(0..jitter);
                total_jitter.fetch_add(jitter_value as usize, Ordering::SeqCst);

                // Print the status of each request with emojis and jitter
                let sleep_duration = duration_per_request + Duration::from_millis(jitter_value);
                let elapsed = start_time.elapsed();

                if sleep_duration > elapsed {
                    thread::sleep(sleep_duration - elapsed);
                }

                // Update the static log output
                let total_requests = success_count.load(Ordering::SeqCst) + fail_count.load(Ordering::SeqCst);
                let average_jitter = total_jitter.load(Ordering::SeqCst) as f64 / total_requests as f64;
                print!("\rTotal Requests: {} | Successful: {} ✅ | Failed: {} ❌ | Average Jitter: {:.2} ms", total_requests, success_count.load(Ordering::SeqCst), fail_count.load(Ordering::SeqCst), average_jitter);
                let _ = std::io::stdout().flush();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let requests = Arc::try_unwrap(requests).unwrap().into_inner().unwrap();

    // Generate and print the final report
    let report = generate_final_report(&requests, config);
    println!("\n{}", report);

    LoadTestReport {
        host: config.host.clone(),
        port: config.port,
        duration: config.duration,
        rps: config.rps,
        load_test_type: config.load_test_type.clone(),
        requests,
    }
}

fn generate_final_report(requests: &[RequestMetrics], config: &Config) -> String {
    let total_requests = requests.len();
    let successful_requests = requests.iter().filter(|r| r.success).count();
    let failed_requests = total_requests - successful_requests;
    let average_response_time: f64 = requests.iter().map(|r| r.response_time).sum::<u64>() as f64 / total_requests as f64;

    format!(
        "\n==================== 📝 Final Report ====================\n\
        📅 Duration: {} seconds\n\
        🏷️ Host: {}\n\
        🚪 Port: {}\n\
        🔄 Requests per Second (RPS): {}\n\
        📦 Payload: {}\n\
        🌐 Load Test Type: {}\n\
        \n\
        📊 **Metrics**:\n\
        - Total Requests: **{}**\n\
        - Successful Requests: **{}** ✅\n\
        - Failed Requests: **{}** ❌\n\
        - Average Response Time: **{:.2}** μs\n\
        \n\
        ==========================================================",
        config.duration, config.host, config.port, config.rps, config.payload, config.load_test_type,
        total_requests, successful_requests, failed_requests, average_response_time
    )
}
