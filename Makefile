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
	cargo build --release --all

.PHONY: build-install
build-install:
	cargo build --release --all
	cargo install --path .
	
.PHONY: cross-build-backend
cross-build-backend:
	cross build --release -p smithe_backend --target aarch64-unknown-linux-gnu

.PHONY: cross-build-frontend
cross-build-frontend:
	cross build --release -p smithe_frontend --target aarch64-unknown-linux-gnu

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

.PHONY: buildx-backend
buildx-backend:
	docker buildx build --platform linux/arm64 -t danstaken/smithe-backend:latest -f Dockerfile-SmitheBackend --push .	

.PHONY: buildx-frontend
buildx-frontend:
	docker buildx build --platform linux/arm64 -t danstaken/smithe-frontend:latest -f Dockerfile-SmitheFrontend --push .

# KUBERNETES
.PHONY: deploy-frontend
deploy-frontend:
	kubectl apply -f ./frontend-deployment.yml
	kubectl apply -f ./frontend-service.yml

# PIDGTM
.PHONY: pidgtm
pidgtm-map:
	$(PIDGTM) map

# BACKEND
.PHONY: run-backend
run-backend:
	cargo run --manifest-path ./backend/Cargo.toml

# FRONTEND
.PHONY: build-frontend
build-frontend:
	trunk build ./frontend/index.html --release

.PHONY: serve-frontend
serve-frontend:
	trunk serve ./frontend/index.html

.PHONY: run-image-backend
run-image-backend:
	node ./image-upload-backend/app.js
