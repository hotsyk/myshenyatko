.PHONY: build run test clean release

build:
	cargo build

run:
	cargo run

test:
ifdef TESTS
	cargo test $(TESTS) -- --nocapture
else
	cargo test
endif

release:
	cargo build --release
	@echo "Binary: target/release/myshenyatko"

clean:
	cargo clean
