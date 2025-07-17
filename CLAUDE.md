# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building and Running
- `cargo run --bin api` - Run the main API service
- `cargo watch -x "run --bin api"` - Run with auto-reload during development
- `cargo build --release` - Build for production
- `cargo fetch` - Install dependencies

### Testing
- `cargo test -p tests` - Run the complete test suite
- `cargo test -p tests -- --nocapture` - Run tests with output

### Database Setup
- `docker-compose up -d surrealdb` - Start SurrealDB for development
- `cargo run --bin seeder` - Run database seeders (optional)

### Production Deployment
- `docker-compose up -d` - Build and run complete stack
- `docker build -t najm-backend .` - Build Docker image

## Code Architecture

This is a Rust monorepo using Cargo workspaces with a microservices architecture:

### Core Services
- **najm-backend**: Main entry point that initializes the gateway
- **najm-gateway**: API gateway that routes requests and provides unified documentation
- **najm-iam**: Identity and Access Management (auth, users, roles, permissions)
- **najm-cms**: Content Management System
- **najm-exam**: Examination system (tests, questions, options, answers, sessions)
- **najm-integration**: External system integrations
- **najm-middleware**: Shared middleware (auth, CORS, permissions)

### Supporting Libraries
- **najm-lib**: Core utilities (Axum, SurrealDB, JWT, Argon2, environment, email)
- **najm-entity**: Shared DTOs and error handling
- **najm-util**: Common utilities (validation, query building, response formatting)
- **tests**: Comprehensive test suite

### Key Patterns
- Each service follows a consistent structure: `controller → service → repository`
- Controllers handle HTTP requests and responses
- Services contain business logic
- Repositories handle database operations
- DTOs are defined separately for request/response validation
- Database schemas are defined per service
- All services use SurrealDB with both WebSocket and in-memory clients

### Technology Stack
- **Web Framework**: Axum with async/await
- **Database**: SurrealDB (dual WebSocket/memory clients)
- **Authentication**: JWT tokens with Argon2 password hashing
- **Documentation**: OpenAPI/Swagger UI (available at `/docs`)
- **Testing**: axum-test for integration testing
- **Validation**: validator crate with derive macros

### Environment Setup
The application requires `.env` file with database connection details, JWT secrets, and port configuration. Use the provided shell scripts (`apply-env.sh` or `apply-env.ps1`) to set environment variables.

### API Structure
- Public routes: `/v1/auth/*`, `/v1/cms/*` (public endpoints)
- Protected routes: All other endpoints require JWT authentication
- Root redirects to `/docs` for API documentation
- OpenAPI spec available at `/openapi.json`

The gateway service (`najm-gateway/src/lib.rs:15`) orchestrates all services and applies authentication middleware to protected routes.