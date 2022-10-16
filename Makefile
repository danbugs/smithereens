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
	cargo test --workspace -- --show-output 2>&1 | tee test.out

.PHONY: build
build:
	cargo build --release

# INSTALL
.PHONY: install
install:
	install target/release/smithe $(INSTALL_DIR)/bin
	install target/release/pidgtm $(INSTALL_DIR)/bin

# PIDGTM
.PHONY: pidgtm
pidgtm-map:
	$(PIDGTM) map

# BACKEND
.PHONY: build-backend
build-backend:
	cargo +nightly build --release --manifest-path ./backend/Cargo.toml

.PHONY: run-backend
run-backend:
	cargo +nightly run --manifest-path ./backend/Cargo.toml

# FRONTEND
.PHONY: serve-frontend
serve-frontend:
	trunk serve ./frontend/index.html --proxy-backend=https://yew.rs/tutorial