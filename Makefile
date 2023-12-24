INSTALL_DIR ?= /usr/local
PIDGTM ?= target/release/pidgtm
SMITHE ?= target/release/smithe

# CARGO
.PHONY: improve
improve:
	cargo clippy --fix -p smithe_backend -p smithe_database -p smithe_lib -p startgg
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
	cargo install --path .

# PIDGTM
.PHONY: pidgtm
pidgtm-map:
	$(PIDGTM) map

.PHONY: pidgtm-user-updater
pidgtm-user-updater:
	cargo build --release --bin pidgtm --target x86_64-unknown-linux-gnu
	docker build -t pidgtm-user-updater -f Dockerfile-UserUpdater .

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
	trunk serve ./frontend/index.html