cargo machete
cargo build --release
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --no-deps
cargo test
