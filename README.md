# Najm Course Backend Service

This repository serves as the **monorepo** for the Najm Course backend services. It encompasses several main services and libraries:

## Core Services

1. **najm-backend** - Main application entry point that initializes the gateway service.
2. **najm-gateway** - Acts as the API gateway, routing requests to appropriate services and providing unified API documentation.
3. **najm-iam** - Handles identity and access management, including authentication, authorization, users, roles, and permissions.
4. **najm-cms** - Content management system services for handling dynamic content.
5. **najm-exam** - Examination system services including tests, questions, options, answers, and sessions.
6. **najm-integration** - Integration services for external systems and APIs.
7. **najm-middleware** - Middleware components for authentication, CORS, and permissions.

## Supporting Libraries

8. **najm-lib** - Core library providing shared utilities for Axum, SurrealDB, JWT, Argon2, environment handling, and email services.
9. **najm-entity** - Shared data transfer objects and error handling entities.
10. **najm-util** - Utility functions for validation, query building, response formatting, and common operations.
11. **tests** - Comprehensive test suite for all services.

## How to Install

1. **Clone the repository**:

   ```sh
   git clone <repository-url>
   cd najmcourse-backend-service-v2
   ```

2. **Set up the environment**:

   - Copy the example environment file:

     ```sh
     cp .env.example .env
     ```

     For Windows-based systems:

     ```sh
     ./apply-env.ps1
     ```

     For Unix-based systems:

     ```sh
     source ./apply-env.sh
     ```

   - Modify the `.env` file with your specific configuration settings including:
     - `PORT` - Application port
     - `SURREALDB_URL` - SurrealDB connection URL
     - `SURREALDB_USERNAME` and `SURREALDB_PASSWORD` - Database credentials
     - `SURREALDB_NAMESPACE` and `SURREALDB_DBNAME` - Database namespace and name
     - `ACCESS_TOKEN_SECRET` and `REFRESH_TOKEN_SECRET` - JWT secrets

3. **Install dependencies**:

   Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, run:

   ```sh
   cargo fetch
   ```

4. **Run the seeders** (optional):

   To populate the database with initial data:

   ```sh
   cargo run --bin seeder
   ```

## How to Run

### Development

To run the services in development mode:

1. **Start the database** using Docker Compose:

   ```sh
   docker-compose up -d surrealdb
   ```

   This will start SurrealDB on port 8000.

2. **Run the main API service**:

   ```sh
   cargo run --bin api
   ```

   The API will be available at `http://localhost:{PORT}` where `{PORT}` is defined in your `.env` file.

3. **Run with auto-reload during development**:

   ```sh
   cargo watch -x "run --bin api"
   ```

4. **Access the API documentation**:

   Once the service is running, visit `http://localhost:{PORT}/docs` for the Swagger UI documentation.

### Production

For production deployment:

1. **Build and run using Docker Compose**:

   ```sh
   docker-compose up -d
   ```

   This will build the application and start both the API service and SurrealDB.

2. **Or build and run manually**:

   ```sh
   docker build -t najm-backend .
   docker run --name najm-backend -d --env-file .env -p ${PORT}:${PORT} najm-backend
   ```

   Make sure to adjust the port mapping according to your `.env` configuration.

## How to Run the Tests

1. **Run the tests**:

   ```sh
   cargo test -p tests
   ```

## How to Contribute

1. **Fork the repository** and clone it locally.
2. **Create a new branch** for your feature or fix:

   ```sh
   git checkout -b feat/your-feature-name
   ```

3. **Make your changes**, commit them, and push to your forked repository.
4. **Create a pull request** to the `develop` branch of this repository.

If you encounter any issues or have questions, feel free to create a new issue in the repository.

## Technology Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: SurrealDB
- **Authentication**: JWT with Argon2 password hashing
- **Documentation**: OpenAPI/Swagger UI
- **Testing**: Cargo test with axum-test
- **Containerization**: Docker & Docker Compose

## API Documentation

When the service is running, comprehensive API documentation is available at:

- **Swagger UI**: `http://localhost:{PORT}/docs`
- **OpenAPI JSON**: `http://localhost:{PORT}/openapi.json`

The API documentation includes all available endpoints, request/response schemas, and authentication requirements.

---

For questions or issues, please create a new issue in this repository.
