.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all

.PHONY: run
run:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo run --release -- --dev --tmp --ws-external --rpc-external

.PHONY: build
build:
	WASM_BUILD_TOOLCHAIN=nightly-2020-10-05 cargo build --release
