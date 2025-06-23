.PHONY: build-frontend build-backend build-all deploy-k8s clean

# Docker image names
FRONTEND_IMAGE = ghcr.io/danbugs/smithe-frontend:latest
BACKEND_IMAGE = ghcr.io/danbugs/smithe-backend:latest

# Build frontend Docker image
build-frontend:
	cd frontend && docker build -f Dockerfile -t $(FRONTEND_IMAGE) .

# Build backend Docker image
build-backend:
	cd backend && docker build -f Dockerfile -t $(BACKEND_IMAGE) .

# Build all Docker images
build-all: build-frontend build-backend

# Push frontend Docker image to GitHub Container Registry
push-frontend:
	docker push $(FRONTEND_IMAGE)

# Push backend Docker image to GitHub Container Registry
push-backend:
	docker push $(BACKEND_IMAGE)

# Push all Docker images to GitHub Container Registry
push-all: push-frontend push-backend

setup-k8s:
	kubectl apply -f k8s/cluster-issuer.yaml

# Deploy to Kubernetes (backend, frontend, ingress).
# - Note: for local ingress setup, see k8s/ingress.local.yaml
deploy-k8s:
	kubectl apply -f k8s/secrets.yaml
	kubectl apply -f k8s/backend.yaml
	kubectl apply -f k8s/frontend.yaml
	kubectl apply -f k8s/ingress.yaml

# Check deployment status
status:
	@echo "=== Pods ==="
	kubectl get pods -l 'app in (smithe-frontend, smithe-backend)'
	@echo "\n=== Services ==="
	kubectl get services
	@echo "\n=== Ingress ==="
	kubectl get ingress nginx

# Tail logs
logs-frontend:
	kubectl logs -f deployment/smithe-frontend

logs-backend:
	kubectl logs -f deployment/smithe-backend

# Delete Kubernetes resources
clean-k8s:
	kubectl delete -f k8s/ingress.yaml --ignore-not-found
	kubectl delete -f k8s/frontend.yaml --ignore-not-found
	kubectl delete -f k8s/backend.yaml --ignore-not-found
	kubectl delete -f k8s/secrets.yaml --ignore-not-found

# Run locally with docker compose
local:
	docker compose up -d --remove-orphans

# Stop local environment
local-stop:
	docker compose down

# Full deployment
deploy: build-all deploy-k8s

# Help
help:
	@echo "Available targets:"
	@echo "  build-frontend    - Build frontend Docker image"
	@echo "  build-backend     - Build backend Docker image"
	@echo "  build-all        - Build all Docker images"
	@echo "  deploy-k8s       - Deploy to Kubernetes"
	@echo "  status           - Check deployment status"
	@echo "  logs-frontend    - Tail frontend logs"
	@echo "  logs-backend     - Tail backend logs"
	@echo "  clean-k8s        - Delete Kubernetes resources"
	@echo "  local            - Run locally with docker-compose"
	@echo "  local-stop       - Stop local environment"
	@echo "  deploy           - Full deployment (build + deploy)"