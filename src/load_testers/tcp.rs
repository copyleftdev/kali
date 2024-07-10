use indicatif::{ProgressBar, ProgressStyle};
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::sleep;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::metrics::{LoadTestReport, RequestMetrics};
use crate::config::Config;

pub async fn perform_load_test(config: &Config) -> LoadTestReport {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let semaphore = Arc::new(Semaphore::new(config.rps as usize));

    let duration_per_request = Duration::from_secs_f64(1.0 / config.rps as f64);
    let progress_bar = ProgressBar::new((config.duration * config.rps as u64) as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%)")
        .expect("Failed to set progress bar template")
        .progress_chars("#>-"));

    let mut tasks = vec![];

    let total_bias: u32 = config.hosts_and_biases.as_ref().map_or(100, |hb| hb.values().sum());

    for _ in 0..config.duration * config.rps as u64 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let requests = Arc::clone(&requests);
        let progress_bar = progress_bar.clone();
        let hosts_and_biases = config.hosts_and_biases.clone();
        let host = config.host.clone();
        let port = config.port;
        let payload = config.payload.clone();
        let jitter = config.jitter;
        let start_time = Instant::now();

        tasks.push(tokio::spawn(async move {
            let mut rng = ChaCha12Rng::from_rng(OsRng).unwrap(); // Thread-safe RNG
            let selected_host = if let Some(hb) = hosts_and_biases {
                let mut selected = None;
                let mut cumulative_bias = 0;
                let random_value = rng.gen_range(0..total_bias);
                for (host, &bias) in &hb {
                    cumulative_bias += bias;
                    if random_value < cumulative_bias {
                        selected = Some(host.clone());
                        break;
                    }
                }
                selected.unwrap_or_else(|| host.clone().unwrap())
            } else {
                host.clone().unwrap()
            };

            let success = match TcpStream::connect((selected_host.as_str(), port)).await {
                Ok(mut stream) => {
                    if stream.write_all(payload.as_bytes()).await.is_ok() {
                        let mut buffer = [0; 1024];
                        if stream.read(&mut buffer).await.is_ok() {
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

            let mut requests = requests.lock().await;
            requests.push(RequestMetrics {
                host: selected_host.clone(),
                response_time,
                success,
                timestamp,
            });

            progress_bar.inc(1);

            drop(permit);
            let jitter_value = rng.gen_range(0..jitter);
            let elapsed = start_time.elapsed();
            let sleep_duration = duration_per_request + Duration::from_millis(jitter_value);
            if sleep_duration > elapsed {
                sleep(sleep_duration - elapsed).await;
            }
        }));
    }

    for task in tasks {
        task.await.unwrap();
    }

    progress_bar.finish_with_message("Load test completed!");

    let requests = Arc::try_unwrap(requests).ok().unwrap().into_inner();

    LoadTestReport {
        metrics: requests,
        duration: config.duration,
        rps: config.rps,
        load_test_type: config.load_test_type.clone(),
    }
}
