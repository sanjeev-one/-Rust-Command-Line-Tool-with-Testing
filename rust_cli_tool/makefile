
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run -- -f ./data.csv -c age -t 20


release:
	cargo build --release

release-arm:
	cargo build --release --arm64



all: format lint test run