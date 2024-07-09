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
    for pair in s.split(',') {
        let parts: Vec<&str> = pair.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid host:bias pair: {}", pair));
        }
        let host = parts[0].to_string();
        let bias: u32 = parts[1].parse().map_err(|_| format!("Invalid bias value: {}", parts[1]))?;
        map.insert(host, bias);
    }
    Ok(map)
}
