mod config;
mod metrics;
mod load_testers;
mod reports;

use clap::Parser;
use config::Config;
use load_testers::tcp::perform_load_test;
use reports::json::output_report_as_json;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    let report = perform_load_test(&config).await;
    output_report_as_json(report, &config.output_file);
}
