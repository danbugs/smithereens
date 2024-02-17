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
.PHONY: install-cert-manager
install-cert-manager:
	kubectl apply -f https://github.com/jetstack/cert-manager/releases/latest/download/cert-manager.yaml

.PHONY: create-clusterissuer
create-clusterissuer:
	kubectl apply -f ./clusterissuer.yml
	# if fails, kubectl delete secret cert-manager-webhook-ca -n cert-manager

.PHONY: install-nginx-ingress
	kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/1bc745619d91b690c8985bbc16097e9fe804d2d2/deploy/static/provider/baremetal/deploy.yaml

.PHONY: get-ingress-ip
get-ingress-ip:
	kubectl get services -o wide -n ingress-nginx
	# ^^^ look for the EXTERNAL_IP to setup DNS

.PHONY: setup-backend-secrets
setup-backend-secrets:
	kubectl create secret generic backend-secrets --from-env-file=backend-secrets.env

.PHONY: deploy-backend
deploy-backend:
	# setup-backend-secrets
	kubectl apply -f ./backend-deployment.yml
	kubectl apply -f ./backend-service.yml

.PHONY: deploy-frontend
deploy-frontend:
	# install-nginx-ingress
	# install-cert-manager
	# create-clusterissuer
	kubectl apply -f ./frontend-deployment.yml
	kubectl apply -f ./frontend-service.yml
	kubectl apply -f ./frontend-ingress.yml

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
	trunk build --release ./frontend/index.html

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