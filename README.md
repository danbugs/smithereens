<img align="right" src="imgs/logo/logo-512x512.png" width="150px" />

# Smithereens 🎮

## Features

- 🏆 **Player Profiles** - View comprehensive player statistics, tournament history, and character usage
- 📊 **Head-to-Head Analysis** - Track your match history against specific opponents
- 📈 **Performance Metrics** - SPR (Seed Performance Rating) calculations and win rate tracking
- 📱 **Responsive Design** - Works well on desktop and mobile devices

## Tech Stack

- **Frontend**: Svelte + Vite + TypeScript
- **Backend**: Rust with Axum
- **Data Source**: start.gg GraphQL API
- **Deployment**: Docker & Docker Compose

## Getting Started

### Prerequisites

- Docker and Docker Compose
- A start.gg API token

### Getting a start.gg API Token

1. Create an account at [start.gg](https://start.gg)
2. Go to your [developer settings](https://start.gg/admin/profile/developer)
3. Generate a new API token
4. Save it for the next step

### Quick Start with Docker Compose

1. **Clone the repository**
   ```bash
   git clone https://github.com/danbugs/smithereens.git
   cd smithereens
   ```

2. **Set up your environment**
   ```bash
   # Create a .env file in the root directory
   echo "STARTGG_TOKEN=Bearer your_token_here" > .env
   ```

3. **Build and start the application**
   ```bash
   # Using make
   make local-build
   make local-run
   ```

4. **Access the application**
   - Frontend: http://localhost:8083
   - Backend API: http://localhost:3000/api

5. **Stop the application**
   ```bash
   make local-stop
   ```

### Development Setup

If you want to run the services locally without Docker:

1. **Backend**
   ```bash
   cd backend
   export SMITHE_STARTGG_TOKEN="Bearer your_token_here"
   export SMITHE_CLIENT_VERSION=20
   export SMITHE_PORT=3000
   cargo run
   ```

2. **Frontend**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

The frontend development server will proxy API requests to the backend automatically.

### Building Docker Images

```bash
# Build both images
make build-all
```

## Project Structure

```
smithereens/
├── frontend/          # Svelte frontend application
│   ├── src/          # Source code
│   ├── Dockerfile    # Frontend container definition
│   └── nginx.conf    # Nginx configuration
├── backend/          # Rust backend API
│   ├── src/          # Source code
│   └── Dockerfile    # Backend container definition
├── docker-compose.yaml
├── Makefile          # Convenient commands
└── README.md
```

## Contributing

We welcome contributions from the community! Whether you're fixing bugs, adding features, or improving documentation, your help makes Smithereens better for everyone.

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow the existing code style
- Write tests for new features
- Update documentation as needed
- Be respectful and constructive in discussions

## Troubleshooting

### Common Issues

1. **Port already in use**: Change the ports in `docker-compose.yaml` if 8083 or 3000 are already taken
2. **API token issues**: Make sure your token includes "Bearer " prefix
3. **Build failures**: Ensure you have the latest Docker version

### Debugging

```bash
# View logs
docker compose logs -f

# Check specific service
docker compose logs -f backend
docker compose logs -f frontend

# Restart services
docker compose restart

# Rebuild images without cache
docker compose build --no-cache
```

## License

This project is licensed under the terms described in the [LICENSE.md](LICENSE.md) file.

## Support

If you encounter any issues or have questions:
- Open an issue on GitHub
- Check existing issues for solutions
- Join our community discussions on our [Discord server](https://discord.gg/QVhHmEdJEx)
