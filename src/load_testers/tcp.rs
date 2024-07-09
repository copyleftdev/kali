use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
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
    let requests = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    let duration_per_request = Duration::from_millis(1000 / config.rps as u64);

    let hosts_and_biases = if let Some(ref hosts_and_biases) = config.hosts_and_biases {
        hosts_and_biases.clone()
    } else {
        let mut map = HashMap::new();
        map.insert(config.host.clone().unwrap(), 100);
        map
    };

    let total_bias: u32 = hosts_and_biases.values().sum();

    let total_requests = (config.duration * config.rps as u64) as u64;
    let progress_bar = ProgressBar::new(total_requests);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%)")
        .expect("Failed to set progress bar template")
        .progress_chars("#>-"));

    for (host, bias) in hosts_and_biases {
        let host_requests = Arc::clone(&requests);
        let port = config.port;
        let payload = config.payload.clone();
        let jitter = config.jitter;
        let progress_bar = progress_bar.clone();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while Instant::now() < end_time {
                let random_value = rng.gen_range(0..total_bias);
                if random_value >= bias {
                    continue;
                }

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

                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

                let mut host_requests = host_requests.lock().unwrap();
                let entry = host_requests.entry(host.clone()).or_insert_with(Vec::new);
                entry.push(RequestMetrics {
                    response_time,
                    success,
                    timestamp,
                });

                progress_bar.inc(1);

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

    progress_bar.finish_with_message("Load test completed!");

    let requests = Arc::try_unwrap(requests).unwrap().into_inner().unwrap();

    let report = generate_final_report(&requests, config);
    println!("\n{}", report);

    LoadTestReport {
        host: config.host.clone().unwrap_or_default(),
        port: config.port,
        duration: config.duration,
        rps: config.rps,
        load_test_type: config.load_test_type.clone(),
        requests: requests.into_iter().flat_map(|(_, v)| v).collect(),
    }
}

fn generate_final_report(requests: &HashMap<String, Vec<RequestMetrics>>, config: &Config) -> String {
    let total_requests: usize = requests.values().map(|v| v.len()).sum();
    let successful_requests: usize = requests.values().map(|v| v.iter().filter(|r| r.success).count()).sum();
    let failed_requests = total_requests - successful_requests;
    let average_response_time: f64 = requests.values().flat_map(|v| v.iter().map(|r| r.response_time)).sum::<u64>() as f64 / total_requests as f64;

    let mut report = format!(
        "\n==================== 📝 Final Report ====================\n\
        📅 Duration: {} seconds\n\
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
        \n",
        config.duration, config.port, config.rps, config.payload, config.load_test_type,
        total_requests, successful_requests, failed_requests, average_response_time
    );

    for (host, metrics) in requests {
        let host_total_requests = metrics.len();
        let host_successful_requests = metrics.iter().filter(|r| r.success).count();
        let host_failed_requests = host_total_requests - host_successful_requests;
        let host_average_response_time: f64 = metrics.iter().map(|r| r.response_time).sum::<u64>() as f64 / host_total_requests as f64;

        report.push_str(&format!(
            "\n📍 **Host: {}**\n\
            - Total Requests: **{}**\n\
            - Successful Requests: **{}** ✅\n\
            - Failed Requests: **{}** ❌\n\
            - Average Response Time: **{:.2}** μs\n",
            host, host_total_requests, host_successful_requests, host_failed_requests, host_average_response_time
        ));
    }

    report.push_str("\n==========================================================");
    report
}
