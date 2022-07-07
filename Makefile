INSTALL_DIR ?= /usr/local
PIDGTM ?= target/release/pidgtm
SSS ?= target/release/sss

# CARGO
.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test -- --show-output

.PHONY: improve
improve:
	cargo clippy --all --all-targets --all-features -- -D warnings
	cargo fmt

#INSTALL
.PHONY: install
install:
	install target/release/sss $(INSTALL_DIR)/bin
	install target/release/pidgtm $(INSTALL_DIR)/bin

# MIGRATIONS
.PHONY: migration-run
migration-run:
	diesel migration run --database-url ${PIDGTM_DATABASE_URL}

.PHONY: migration-redo
migration-redo:
	diesel migration redo --database-url ${PIDGTM_DATABASE_URL}

.PHONY: pidgtm
pidgtm-map:
	$(PIDGTM) map

