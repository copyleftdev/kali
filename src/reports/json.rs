use serde_json;
use std::fs::File;
use std::io::Write;
use crate::metrics::LoadTestReport;

pub fn output_report_as_json(report: LoadTestReport, output_file: &str) {
    let json_report = serde_json::to_string_pretty(&report).unwrap();
    let mut file = File::create(output_file).unwrap();
    file.write_all(json_report.as_bytes()).unwrap();
}
