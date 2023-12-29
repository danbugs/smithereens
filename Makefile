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

.PHONY: install
install:
	cargo install --path .

.PHONY: build
build:
	cargo build --release

.PHONY: build-install
build-install:
	cargo build --release
	cargo install --path .
	
.PHONY: cross-build
cross-build:
	cross build --release --target aarch64-unknown-linux-gnu

# DOCKER
.PHONY: buildx-rsbuildenvarm64
buildx-rsbuildenvarm64:
	docker buildx build --platform linux/arm64 -t danstaken/rust-build-env-arm64:latest -f Dockerfile-RustBuildEnvArm64 --push .

.PHONY: buildx-pidgtm
buildx-pidgtm:
	docker buildx build --platform linux/arm64 -t danstaken/pidgtm:latest -f Dockerfile-Pidgtm --push .

# PIDGTM
.PHONY: pidgtm
pidgtm-map:
	$(PIDGTM) map

# BACKEND
.PHONY: run-backend
run-backend:
	cargo run --manifest-path ./backend/Cargo.toml

# FRONTEND
.PHONY: serve-frontend
serve-frontend:
	trunk serve ./frontend/index.html

# PYTHON
.PHONY: py-install-reqs
py-install-reqs:
	py -m pip install -r TokenGenerationBot-requirements.txt

.PHONY: py-run
py-run:
	py TokenGenerationBot.py