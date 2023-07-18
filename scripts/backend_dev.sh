#!/bin/bash

set -e

echo "Starting local docker Postgres db named 'demodb'"
docker run --name dev_demo_db -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_DB=demodb -p 5432:5432 -d postgres

echo "Pausing while db starts"
sleep 2 # hack

echo "migrating database"
sqlx migrate run