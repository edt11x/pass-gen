#!/bin/bash
# Script to run the pass-gen application

# Ensure we are in the project directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

echo "Starting Password Generator..."
cargo run --quiet
