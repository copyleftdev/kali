use clap::Parser;

#[derive(Parser, Debug)]
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
