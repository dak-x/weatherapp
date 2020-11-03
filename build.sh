#!/bin/bash
# This sets the token as an env variable
export OPENWEATHERAPI=$(<token.txt)
cargo build --release 
