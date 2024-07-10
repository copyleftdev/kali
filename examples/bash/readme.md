# Load Testing Scripts

This folder contains bash scripts for performing different types of load tests on a specified TCP server using the `kali` tool. Each script generates a JSON report with the results.

## Prerequisites

Ensure that `kali` has been built and is either in the same current working directory (CWD) or in your `$PATH`.

## Test Types

### Stress Test

This script performs a stress test by generating a high number of requests per second (RPS) for a given duration to evaluate the server's performance under heavy load.

#### Usage

```sh
./stress_test.sh --host 192.168.0.109 --port 8080 --duration 60 --rps 1000 --payload "Hello World" --jitter 50
```

### Spike Test

This script performs a spike test by generating a sudden burst of requests to simulate a spike in traffic and measure the server's response.

#### Usage

```sh
./spike_test.sh --host 192.168.0.109 --port 8080 --duration 10 --rps 5000 --payload "Hello World" --jitter 50
```

### Soak Test

This script performs a soak test by generating a moderate number of requests per second (RPS) over an extended period to measure the server's long-term stability and performance.

#### Usage

```sh
./soak_test.sh --host 192.168.0.109 --port 8080 --duration 3600 --rps 100 --payload "Hello World" --jitter 50
```

### Volume Test

This script performs a volume test by generating a high number of requests per second (RPS) for a given duration to evaluate the server's ability to handle a large volume of traffic.

#### Usage

```sh
./volume_test.sh --host 192.168.0.109 --port 8080 --duration 300 --rps 1000 --payload "Hello World" --jitter 50
```

## Parameters

- `--host`: The IP address or hostname of the TCP server.
- `--hosts-and-biases`: A comma-separated list of hosts and biases in the format `HOST1:BIAS1,HOST2:BIAS2`. (Optional)
- `--port`: The port number of the TCP server.
- `--duration`: The duration of the test in seconds.
- `--rps`: The number of requests per second to be generated.
- `--payload`: The payload to be sent with each request.
- `--jitter`: The maximum jitter (in milliseconds) to be added to the sleep duration between requests. Default is 50ms.

## Output

The results of each test are saved in a JSON file with a timestamp, e.g., `test_type_output_20230710153045.json`.

## Example Commands

### Stress Test

```sh
./stress_test.sh --host 192.168.0.109 --port 8080 --duration 60 --rps 1000 --payload "Hello World" --jitter 50
```

### Spike Test

```sh
./spike_test.sh --host 192.168.0.109 --port 8080 --duration 10 --rps 5000 --payload "Hello World" --jitter 50
```

### Soak Test

```sh
./soak_test.sh --host 192.168.0.109 --port 8080 --duration 3600 --rps 100 --payload "Hello World" --jitter 50
```

### Volume Test

```sh
./volume_test.sh --host 192.168.0.109 --port 8080 --duration 300 --rps 1000 --payload "Hello World" --jitter 50
```
