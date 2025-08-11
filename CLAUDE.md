# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Environment Setup
- `cp .env.example .env` - Copy environment template
- `source ./apply-env.sh` - Apply environment variables (Unix)
- `./apply-env.ps1` - Apply environment variables (Windows)
- `cargo fetch` - Install dependencies

### Building and Running
- `cargo run --bin api` - Run the main API service
- `cargo watch -x "run --bin api"` - Run with auto-reload during development
- `cargo build --release` - Build for production

### Testing
- `cargo test -p tests` - Run the complete test suite
- `cargo test -p tests -- --nocapture` - Run tests with output
- `cargo test -p tests {module_name}` - Run specific test module
- `cargo test -p tests -- --test-threads=1` - Run tests sequentially

### Database Setup
- `docker-compose up -d surrealdb` - Start SurrealDB for development
- `cargo run --bin seeder` - Run database seeders (optional)

### Code Quality
- `cargo fmt` - Format code according to rustfmt.toml (hard tabs, 85 char width)
- `cargo clippy` - Run linter checks
- `cargo check` - Quick compile check without building

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
- Controllers handle HTTP requests and responses using Axum extractors
- Services contain business logic and coordinate between repositories
- Repositories handle database operations using SurrealDB queries
- DTOs are defined separately for request/response validation using serde and validator
- Database schemas are defined per service using surrealdb::sql::Thing
- All services use SurrealDB with both WebSocket and in-memory clients
- Feature modules follow naming: `{feature}_{type}.rs` (e.g., `users_controller.rs`)
- Permission-based authorization using `permissions_guard` middleware
- Consistent error handling with anyhow::Result in repositories

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

## Development Workflow

### Adding New Features
1. **Module Structure**: Create new features in `src/v1/{feature}/` following the established pattern
2. **File Naming**: Use `{feature}_{type}.rs` convention (controller, service, repository, dto, schema)
3. **Router Integration**: Add feature router to service's `mod.rs` and include in main router
4. **Database Operations**: Use `QueryListBuilder` for list operations, `Thing` for IDs
5. **Testing**: Add corresponding tests in `tests/` workspace following same structure

### SurrealDB Patterns
- Use `ResourceEnum` for consistent table naming
- Implement soft deletes with `is_deleted` field
- Use `FETCH` in queries to include related entities
- Handle ID extraction with pattern matching on `surrealdb::sql::Id::String`
- Use `make_thing()` utility for creating Thing IDs

### Authentication Flow
- JWT tokens managed through `najm-lib/jsonwebtoken`
- Permission checking via `permissions_guard()` with `PermissionsEnum`
- User context extracted from headers in authenticated endpoints
- Argon2 for password hashing with salt

### Response Patterns
- Use `ResponseSuccessDto<T>` for single items
- Use `ResponseListSuccessDto<T>` for paginated lists with meta
- Consistent error responses via `common_response()` utility
- All responses include version field ("0.1.0")