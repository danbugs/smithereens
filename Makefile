SSS_INSTALL_DIR ?= /usr/local

build:
	cargo build --release

test:
	cargo test -- --show-output

install:
	install target/release/sss $(INSTALL_DIR)/bin