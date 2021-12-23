build:
	cargo build --release --all-features

lint:  # fix code
	cargo clippy --all-features
	cargo fmt --all -- --check

fix:  # fix code
	cargo fmt --all

tests:  # test
	cargo test $CARGO_OPTIONS -- -Z unstable-options --format json | cargo2junit > results.xml;
