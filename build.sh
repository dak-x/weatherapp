#!/bin/bash
# Builds the application
echo "const APP_ID:&str = \"$(<token.txt)\";" >> src/lib.rs
cargo build --release 
