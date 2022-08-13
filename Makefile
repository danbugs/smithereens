INSTALL_DIR ?= /usr/local
PIDGTM ?= target/release/pidgtm
SMITHE ?= target/release/smithe

# CARGO
.PHONY: improve
improve:
	cargo clippy --all --all-targets --all-features -- -D warnings
	cargo fmt --all

.PHONY: test
test:
	cargo test -- --show-output

.PHONY: build
build:
	cargo build --release

#INSTALL
.PHONY: install
install:
	install target/release/smithe $(INSTALL_DIR)/bin
	install target/release/pidgtm $(INSTALL_DIR)/bin

# MIGRATIONS
.PHONY: migration-run
migration-run:
	diesel migration run --database-url ${PIDGTM_DATABASE_URL}

.PHONY: migration-redo
migration-redo:
	diesel migration redo --database-url ${PIDGTM_DATABASE_URL}

.PHONY: migration-revert
migration-revert:
	diesel migration revert --database-url ${PIDGTM_DATABASE_URL}

.PHONY: pidgtm
pidgtm-map:
	$(PIDGTM) map

