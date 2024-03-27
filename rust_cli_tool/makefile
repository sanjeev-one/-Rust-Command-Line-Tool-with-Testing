
format:
	cargo fmt 

lint:
	cargo clippy 

test:
	cargo test 

run:
	cargo run -- -f ./data.csv -c age -t 20


release:
	cargo build --release

release-arm:
	cargo build --release --arm64



all: format lint test run