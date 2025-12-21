
.PHONY: clean upgrade format check build build-all rebuild release test build_test

clean:
	cargo clean && rm -f Cargo.lock
upgrade:
	cargo upgrade
format:
	cargo +nightly fmt
check:
	cargo check
build:
	cargo build

rebuild: clean build

release:
	cargo build --release
test:
	cd rust && cargo test
	cd demo/ffi && cargo test
	cd rust_kits && cargo test
	cd crates/dashmap_ && cargo test
build_test:
	cargo test --no-run
