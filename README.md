# 🚀 REST API Starter Kit with Axum, SQLx, and SQLite

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)![SQLx](https://img.shields.io/badge/sqlx-0.7-green.svg)![SQLite](https://img.shields.io/badge/sqlite-embedded-blue.svg)![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)

</div>

Welcome to the **Axum**-based REST API Starter Kit, a robust, modern, and production-ready foundation for building backend services with Rust. This project now uses **SQLite** for ease of deployment without sacrificing performance for medium scale.

## ✨ Key Features

-   ✅ **Modern Web Framework**: Built on top of the fast and ergonomic [Axum](https://github.com/tokio-rs/axum).
-   🔒 **JWT Authentication**: Complete implementation for *Register*, *Login*, *Logout*, and *Refresh Token*.
-   🔐 **Security**: *Password Hashing* using `bcrypt`.
-   🏢 **Layered Architecture**: Clear separation between business logic (*Usecase*), data access (*Repository*), and API routes (*Handler*).
-   👤 **User & Role Management**: CRUD for users with a role system (*Admin* & *User*).
-   🛡️ **Middleware & Protected Routes**: Examples of using *middleware* for authentication and role-based authorization.
-   🗃️ **SQLite Database**: Lightweight, *embedded*, and does not require a separate database server.
-   📚 **Automatic API Documentation**: Swagger UI integrated with [Utoipa](https://github.com/juhaku/utoipa).
-   ⚙️ **Flexible Configuration**: Configuration management via `.env` file and *environment variables*.
-   📝 **Logging**: Structured logging with `tracing` to facilitate *debugging*.
-   🐳 **Docker Support**: Extremely lightweight and easy to run (self-contained).
-   🧪 **API Testing**: Equipped with Python scripts for endpoint testing as a Postman replacement.

## 📁 Project Structure

Standard Rust structure, with the addition of the `.sqlx/` folder for *offline compilation* and `data/` (created at runtime) to store database files.

```
.
├── api_tests/              # API testing scripts (Postman replacement)
├── migrations/             # SQLx database migration files
├── src/
│   ├── config/             # Application configuration module
│   ├── domain/             # Data models and main structs
│   ├── error/              # Custom error handling
│   ├── handler/            # Logic to handle HTTP requests
│   ├── middleware/         # Authentication & authorization middleware
│   ├── repository/         # Data access logic for database
│   ├── routes/             # API route definitions and wiring
│   └── usecase/            # Core application business logic
├── .env                    # Configuration file for local development (NOT committed)
├── .env.docker             # Configuration file for Docker
├── .sqlx/                  # SQLx metadata cache for offline mode
├── Cargo.toml              # Project dependencies
└── Dockerfile              # Recipe for building Docker image
```

## 🚀 Getting Started

We recommend running this project locally first to understand the flow. If you encounter difficulties with local setup, Docker is an excellent alternative.

### 👩‍💻 Method 1: Local Development (Recommended)

#### Prerequisites
-   [Rust](https://www.rust-lang.org/tools/install) (version 1.89+).
-   [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) with sqlite feature.
    ```sh
    cargo install sqlx-cli --no-default-features --features rustls,sqlite --force
    ```

#### Setup Steps
1.  **Clone Repository**
    ```sh
    git clone https://github.com/mnabielap/starter-kit-restapi-axum.git
    cd starter-kit-restapi-axum
    ```

2.  **Create `.env` File**
    Create a file named `.env` in the project root and copy the following content. Adjust if necessary.
    ```env
    # .env
    DATABASE_URL=sqlite://data.db?mode=rwc
    SERVER_HOST=127.0.0.1
    SERVER_PORT=8000

    # JWT Secrets
    JWT_SECRET=local_secret_123
    JWT_ACCESS_TOKEN_EXPIRES_IN=15m
    JWT_REFRESH_TOKEN_EXPIRES_IN=7d
    ```

3.  **Setup Database & Migrations**
    This command will create the `data.db` file and create tables.
    ```sh
    # Create database file
    sqlx database create

    # Run migrations
    sqlx migrate run
    ```
    > 💡 **Important:** If you change SQL queries within the code, run `cargo sqlx prepare` to update the `.sqlx` cache.

4.  **Run Application**
    ```sh
    cargo run
    ```
    🎉 Your server is now running at `http://127.0.0.1:8000`.

---

### 🐳 Method 2: Using Docker

Since it uses SQLite, we do not need to run a separate database container. The application becomes *self-contained*.

#### Prerequisites
-   [Docker](https://www.docker.com/get-started) installed.
-   **IMPORTANT**: Before building docker, ensure SQLx metadata is up to date to compile without a DB connection inside the Docker builder.
    ```sh
    # Run this on your local machine before building docker
    cargo sqlx prepare
    ```

#### Setup Steps
1.  **Build Application Image**
    ```sh
    docker build -t restapi-axum-sqlite .
    ```

2.  **Create `.env.docker` File**
    The application inside Docker will use this file for configuration.
    ```env
    # .env.docker
    # Note: 'data' refers to the folder inside the container working directory
    DATABASE_URL=sqlite://data/data.db?mode=rwc
    SERVER_HOST=0.0.0.0
    SERVER_PORT=8000
    JWT_SECRET=this_is_a_very_secure_and_very_long_jwt_secret_for_development_in_docker
    JWT_ACCESS_TOKEN_EXPIRES_IN=15m
    JWT_REFRESH_TOKEN_EXPIRES_IN=7d
    ```

3.  **Create Volume for Data (Optional but Recommended)**
    To ensure user data is not lost when the container is removed, we create a volume.
    ```sh
    docker volume create axum_sqlite_data
    ```

4.  **Run Container**
    We will mount the volume to `/app/data` and use the `.env.docker` file.
    ```sh
    docker run -d \
      -p 5005:8000 \
      --name axum-app \
      --env-file .env.docker \
      -v axum_sqlite_data:/app/data \
      --restart always \
      restapi-axum-sqlite
    ```
    🎉 Your server is now running inside Docker and can be accessed at `http://localhost:5005`.

    > *Note: Migrations should ideally be run by the application at startup, or you can mount a pre-migrated `.db` file.*

## 🧪 API Testing (Postman Replacement)

The `api_tests/` directory contains simple Python scripts to test each endpoint. This is very useful for quick verification without needing to open Postman.

#### Prerequisites
-   [Python 3](https://www.python.org/downloads/)
-   `requests` library.
    ```sh
    pip install requests
    ```

#### How to Run Tests
Simply run the relevant Python file from the terminal. The script will print the API response output.
```sh
# Example running registration test
python api_tests/1.auth_register.py

# Example running login test
python api_tests/3.auth_login.py
```
> 💡 These scripts default to targeting `http://localhost:8000`. If you are using Docker, change the URL in the script to `http://localhost:5005`.

## 📚 API Documentation (Swagger UI)

This project automatically generates OpenAPI documentation. You can access it via browser to see all available endpoints and try them directly.

-   **Local Development**: [http://localhost:8000/swagger-ui](http://localhost:8000/swagger-ui)
-   **Docker**: [http://localhost:5005/swagger-ui](http://localhost:5005/swagger-ui)

## ⚙️ Docker Container Management (Cheat Sheet)

Here are some useful Docker commands to manage your containers.

-   🪵 **View logs from a running container**
    ```sh
    docker logs -f axum-app
    ```
-   🛑 **Stop container**
    ```sh
    docker stop axum-app
    ```
-   ▶️ **Start an existing container**
    ```sh
    docker start axum-app
    ```
-   🗑️ **Remove container (after stopping)**
    ```sh
    docker rm axum-app
    ```
-   🗂️ **List existing volumes**
    ```sh
    docker volume ls
    ```
-   ⚠️ **Remove volume (WARNING: This will permanently delete data!)**
    ```sh
    # Careful when running this command!
    docker volume rm axum_sqlite_data
    ```

## 🛠️ Main Technologies & Libraries

-   **Framework**: [Axum](https://github.com/tokio-rs/axum)
-   **Async Runtime**: [Tokio](https://tokio.rs/)
-   **Database ORM/Toolkit**: [SQLx](https://github.com/launchbadge/sqlx)
-   **Serialization/Deserialization**: [Serde](https://serde.rs/)
-   **API Documentation**: [Utoipa](https://github.com/juhaku/utoipa)
-   **Configuration**: [config-rs](https://github.com/mehcode/config-rs), [dotenvy](https://github.com/allan2/dotenvy)
-   **Validation**: [validator](https://github.com/Keats/validator)
-   **JWT**: [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
-   **Logging**: [tracing](https://github.com/tokio-rs/tracing)

## 📄 License

This project is licensed under the [MIT License](LICENSE).