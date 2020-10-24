# 14da5e4cac40d2e8893248d960ce48b6
#!/bin/bash
# Builds the application
echo "const APP_ID:&str = \"$(<token.txt)\";" >> src/lib.rs
cargo build --release 
