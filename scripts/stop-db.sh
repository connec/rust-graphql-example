#!/bin/bash

set -euo pipefail

container_name=sample-db

if ! output="$(docker stop "$container_name" 2>&1)"; then
  exit_code=$?
  echo "Could not stop $container_name container" >&2
  echo >&2
  echo "$output" >&2
  exit $exit_code
fi

if ! output="$(docker rm "$container_name" 2>&1)"; then
  exit_code=$?
  echo "Could not remove $container_name container" >&2
  echo >&2
  echo "$output" >&2
  exit $exit_code
fi

echo "Stopped container $container_name" >&2
