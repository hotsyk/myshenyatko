.PHONY: build run test clean

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

clean:
	cargo clean
