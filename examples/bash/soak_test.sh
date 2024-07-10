#!/bin/sh

# Usage: ./soak_test.sh --host 192.168.0.109 --port 8080 --duration 3600 --rps 100 --payload "Hello World" --jitter 50

timestamp=$(date +%Y%m%d%H%M%S)
output_file="soak_output_${timestamp}.json"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --host) HOST="$2"; shift 2;;
    --hosts-and-biases) HOSTS_AND_BIASES="$2"; shift 2;;
    --port) PORT="$2"; shift 2;;
    --duration) DURATION="$2"; shift 2;;
    --rps) RPS="$2"; shift 2;;
    --payload) PAYLOAD="$2"; shift 2;;
    --jitter) JITTER="$2"; shift 2;;
    *) echo "Unknown parameter passed: $1"; exit 1;;
  esac
done

kali --host "$HOST" --hosts-and-biases "$HOSTS_AND_BIASES" --port "$PORT" --duration "$DURATION" --rps "$RPS" --load-test-type tcp --output-file "$output_file" --payload "$PAYLOAD" --jitter "$JITTER"
