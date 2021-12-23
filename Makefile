dev:
	rustup component add rustfmt
	rustup component add clippy
	cargo install cargo2junit
	cargo install grcov
	rustup component add llvm-tools-preview

build:
	cargo build --release --all-features

lint:  # fix code
	cargo clippy --all-features
	cargo fmt --all -- --check

fix:  # fix code
	cargo fmt --all

tests:  # test
	cargo test -- -Z unstable-options --format json | cargo2junit > results.xml
