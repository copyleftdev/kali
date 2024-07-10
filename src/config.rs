use clap::{Parser, ArgGroup};
use std::collections::HashMap;

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
#[command(disable_help_flag = true)]
#[command(group(ArgGroup::new("hosts").required(true).args(&["host", "hosts_and_biases"])))]
pub struct Config {
    #[arg(short = 'H', long)]
    pub host: Option<String>,

    #[arg(long, value_parser=parse_hosts_and_biases, value_name="HOST1:BIAS1,HOST2:BIAS2")]
    pub hosts_and_biases: Option<HashMap<String, u32>>,

    #[arg(short = 'p', long)]
    pub port: u16,

    #[arg(short, long)]
    pub duration: u64,

    #[arg(short, long)]
    pub rps: u32,

    #[arg(short, long)]
    pub load_test_type: String,

    #[arg(short, long)]
    pub output_file: String,

    #[arg(short = 'P', long)]
    pub payload: String,

    #[arg(short, long, default_value_t = 50)]
    pub jitter: u64,

    #[arg(short, long, action = clap::ArgAction::Help)]
    pub help: Option<bool>,
}

fn parse_hosts_and_biases(s: &str) -> Result<HashMap<String, u32>, String> {
    let mut map = HashMap::new();
    let mut total_bias: u64 = 0; // Using u64 to prevent overflow when summing biases
    for pair in s.split(',') {
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid host:bias pair: {}", pair));
        }
        let host = parts[0].to_string();
        let bias: u32 = parts[1].parse().map_err(|_| format!("Invalid bias value: {}", parts[1]))?;
        if bias == 0 {
            return Err(format!("Bias value must be greater than 0: {}", parts[1]));
        }
        total_bias = total_bias.checked_add(bias as u64).ok_or("Total bias exceeds allowable limit")?;
        map.insert(host, bias);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::Config;
    use clap::Parser;
    use std::collections::HashMap;

    #[test]
    fn test_config_parsing() {
        let args = vec![
            "kali",
            "--hosts-and-biases", "127.0.0.1:70,192.168.1.1:30",
            "--port", "8080",
            "--duration", "10",
            "--rps", "100",
            "--load-test-type", "tcp",
            "--output-file", "output.json",
            "--payload", "Hello World",
            "--jitter", "100",
        ];
        let config = Config::parse_from(args);
        let mut expected_hosts_and_biases = HashMap::new();
        expected_hosts_and_biases.insert("127.0.0.1".to_string(), 70);
        expected_hosts_and_biases.insert("192.168.1.1".to_string(), 30);
        
        assert_eq!(config.hosts_and_biases, Some(expected_hosts_and_biases));
        assert_eq!(config.port, 8080);
        assert_eq!(config.duration, 10);
        assert_eq!(config.rps, 100);
        assert_eq!(config.load_test_type, "tcp");
        assert_eq!(config.output_file, "output.json");
        assert_eq!(config.payload, "Hello World");
        assert_eq!(config.jitter, 100);
    }

    #[test]
    fn test_invalid_bias_parsing() {
        let args = vec![
            "kali",
            "--hosts-and-biases", "127.0.0.1:70,192.168.1.1:0",
            "--port", "8080",
            "--duration", "10",
            "--rps", "100",
            "--load-test-type", "tcp",
            "--output-file", "output.json",
            "--payload", "Hello World",
            "--jitter", "100",
        ];
        let result = Config::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_bias_overflow_parsing() {
        let args = vec![
            "kali",
            "--hosts-and-biases", "127.0.0.1:4294967295,192.168.1.1:1",
            "--port", "8080",
            "--duration", "10",
            "--rps", "100",
            "--load-test-type", "tcp",
            "--output-file", "output.json",
            "--payload", "Hello World",
            "--jitter", "100",
        ];
        let result = Config::try_parse_from(args);
        assert!(result.is_err());
    }
}
