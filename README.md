

# Kali


![Kali Logo](./logo.png)

Kali is a high-concurrency TCP load testing tool designed to generate a customizable number of requests per second (RPS) to a specified TCP server, while collecting performance metrics and outputting the results in a JSON report. The tool is highly configurable, allowing users to specify various parameters such as host, port, duration, payload, and jitter.

## Meaning Behind the Name

The name "Kali" is derived from the Hindu goddess Kali, who is associated with destruction and transformation. Kali embodies the power to bring chaos and destruction, but this force is also seen as necessary for rebirth and renewal. Similarly, the Kali load testing tool can be seen as a force that puts stress and load on a system (destruction), which helps in identifying weaknesses and improving the system's resilience and performance (rebirth and renewal).

## How It Achieves Requests Per Second (RPS)

Kali achieves the desired RPS by:
1. **Spawning Multiple Threads**: For each RPS, a separate thread is spawned. This ensures that the load is distributed across multiple threads, allowing for high concurrency.
2. **Controlled Sleep Duration**: Each thread calculates the sleep duration between requests based on the specified RPS value. This duration is randomized using a jitter value to simulate more realistic network conditions.
3. **Precise Timing**: By sleeping for the calculated duration, each thread ensures that requests are sent at the desired rate, accounting for any random jitter introduced.

## Project Structure

The project is organized into several modules, each responsible for a specific aspect of the tool's functionality. Here's an overview of the modules:

```
src/
├── main.rs         # Entry point of the application
├── config.rs       # Configuration parsing and management
├── metrics.rs      # Data structures for metrics collection
├── load_testers/   # Load testing implementations
│   ├── mod.rs      # Module declarations
│   └── tcp.rs      # TCP load testing implementation
├── reports/        # Reporting implementations
│   ├── mod.rs      # Module declarations
│   └── json.rs     # JSON report generation
Cargo.toml          # Project dependencies and metadata
```

### Detailed Module Descriptions

1. **main.rs**
   - The entry point of the application.
   - Parses command-line arguments and initializes the configuration.
   - Invokes the load testing and report generation functions.

2. **config.rs**
   - Defines the `Config` struct to hold configuration parameters.
   - Uses the `clap` crate to parse command-line arguments and populate the `Config` struct.

3. **metrics.rs**
   - Defines data structures (`RequestMetrics` and `LoadTestReport`) for collecting and serializing performance metrics.
   - Metrics include response time, success status, and timestamps.

4. **load_testers/mod.rs**
   - Module declaration for load testers.
   - Provides an interface for different types of load testers.

5. **load_testers/tcp.rs**
   - Implements TCP load testing.
   - Manages the concurrency and timing of requests to achieve the desired RPS.
   - Collects performance metrics for each request.

6. **reports/mod.rs**
   - Module declaration for report generation.
   - Provides an interface for different types of reports.

7. **reports/json.rs**
   - Implements JSON report generation.
   - Serializes the collected metrics into a JSON format and writes to the specified output file.

## Usage

### Command-Line Arguments

- `--host <HOST>`: The IP address or hostname of the TCP server.
- `--port <PORT>`: The port number of the TCP server.
- `--duration <DURATION>`: The duration of the load test in seconds.
- `--rps <RPS>`: The number of requests per second to be generated.
- `--load-test-type <LOAD_TEST_TYPE>`: The type of load test (e.g., "tcp").
- `--output-file <OUTPUT_FILE>`: The file where the JSON report will be saved.
- `--payload <PAYLOAD>`: The payload to be sent with each request.
- `--jitter <JITTER>`: The maximum jitter (in milliseconds) to be added to the sleep duration between requests (default: 50).

### Example Command

```bash
cargo run -- --host 127.0.0.1 --port 8080 --duration 10 --rps 100 --load-test-type tcp --output-file output.json --payload "Hello World" --jitter 100
```

## Contributing

### Adding New Load Testers

To add a new type of load tester:
1. Create a new file in the `load_testers` directory (e.g., `http.rs`).
2. Implement the load testing logic in the new file.
3. Update `load_testers/mod.rs` to include the new load tester module.

### Adding New Report Formats

To add a new report format:
1. Create a new file in the `reports` directory (e.g., `csv.rs`).
2. Implement the report generation logic in the new file.
3. Update `reports/mod.rs` to include the new report module.

### Patterns for Contribution

- **Modular Design**: Ensure each new feature or component is added as a separate module to maintain the modularity of the codebase.
- **Clear Interfaces**: Define clear interfaces for new load testers and report formats to ensure they can be easily integrated with the existing codebase.
- **Documentation**: Document new modules and features thoroughly to help future contributors understand the code and make further enhancements.
---

Kali is a powerful tool for load testing TCP servers, designed with flexibility and performance in mind. We welcome contributions and feedback from the community to make Kali even more robust and versatile.