INSTALL_DIR ?= /usr/local

build:
	cargo build --release

test:
	cargo test -- --show-output

install:
	install target/release/smithe $(INSTALL_DIR)/bin