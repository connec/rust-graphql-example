#!/bin/bash

set -euo pipefail

container_name=sample-db
container_image=postgres:14

if ! [[ "$(docker ps --filter name="$container_name" --latest -q)" != '' ]]; then
  if
    ! output="$(
      docker run \
        --detach \
        --env POSTGRES_PASSWORD="$PGPASSWORD" \
        --name "$container_name" \
        --publish "$PGPORT:$PGPORT" \
        "$container_image" \
      2>&1
    )"
  then
    exit_code=$?
    echo "Failed to start database container" >&2
    echo "$output" >&2
    exit $exit_code
  fi
fi

attempts=0
while ! output="$(docker exec -e PGHOST -e PGPORT -e PGUSER -e PGPASSWORD sample-db psql -c 'SELECT 1' 2>&1)"; do
  exit_code=$?

  printf '.' >&2

  attempts=$((attempts + 1))
  if [[ $attempts -ge 10 ]]; then
    echo ' Database did not become responsive in time' >&2
    echo 'Last output:' >&2
    echo >&2
    echo "$output" >&2
    exit $exit_code
  fi

  sleep 1
done

if [[ $attempts -gt 0 ]]; then
  printf ' ' >&2
fi
echo "Database container sample-db ($(docker ps -q --filter name="$container_name")) is running!" >&2
