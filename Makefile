lint:
	cargo clippy --all-targets -- -D warnings
	cargo fmt -- --check

build:
	cargo build --release

