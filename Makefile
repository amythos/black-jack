

VERSION := 0.0.1

build-docs:
	cargo doc --no-deps --open

build:
	cargo build

coverage:
	cargo tarpaulin -v

test:
	cargo test