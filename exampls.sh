cargo run -- --host 192.168.0.109 --port 8080 --duration 10 --rps 100 --load-test-type tcp --output-file output.json --payload "Hello World" --jitter 100
cargo run -- --hosts-and-biases "192.168.0.109:70,192.168.1.1:30" --port 8080 --duration 10 --rps 100 --load-test-type tcp --output-file output.json --payload "Hello World" --jitter 100
