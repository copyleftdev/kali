use clap::Parser;

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
#[command(disable_help_flag = true)]
pub struct Config {
    #[arg(short = 'H', long)]
    pub host: String,

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

#[cfg(test)]
mod tests {
    use super::Config;
    use clap::Parser;

    #[test]
    fn test_config_parsing() {
        let args = vec![
            "kali",
            "--host", "127.0.0.1",
            "--port", "8080",
            "--duration", "10",
            "--rps", "100",
            "--load-test-type", "tcp",
            "--output-file", "output.json",
            "--payload", "Hello World",
            "--jitter", "100",
        ];
        let config = Config::parse_from(args);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.duration, 10);
        assert_eq!(config.rps, 100);
        assert_eq!(config.load_test_type, "tcp");
        assert_eq!(config.output_file, "output.json");
        assert_eq!(config.payload, "Hello World");
        assert_eq!(config.jitter, 100);
    }
}
