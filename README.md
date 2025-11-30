# 🚀 REST API Starter Kit with Axum, SQLx, and Postgres

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)![SQLx](https://img.shields.io/badge/sqlx-0.7-green.svg)![PostgreSQL](https://img.shields.io/badge/postgres-16-blue.svg)![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)

</div>

Welcome to the **Axum**-based REST API Starter Kit, a robust, modern, and production-ready foundation for building backend services with Rust. This project is designed with a layered architecture (*Usecase*, *Repository*, *Handler*) to ensure organized, testable, and maintainable code.

## ✨ Key Features

-   ✅ **Modern Web Framework**: Built on top of the fast and ergonomic [Axum](https://github.com/tokio-rs/axum).
-   🔒 **JWT Authentication**: Complete implementation for *Register*, *Login*, *Logout*, and *Refresh Token*.
-   🔐 **Security**: *Password Hashing* using `bcrypt`.
-   🏢 **Layered Architecture**: Clear separation between business logic (*Usecase*), data access (*Repository*), and API routes (*Handler*).
-   👤 **User & Role Management**: CRUD for users with a role system (*Admin* & *User*).
-   🛡️ **Middleware & Protected Routes**: Examples of using *middleware* for authentication and role-based authorization.
-   🐘 **Postgres Database**: Uses [SQLx](https://github.com/launchbadge/sqlx) with *compile-time checked queries* for type safety.
-   📚 **Automatic API Documentation**: Swagger UI integrated with [Utoipa](https://github.com/juhaku/utoipa).
-   ⚙️ **Flexible Configuration**: Configuration management via `.env` file and *environment variables*.
-   📝 **Logging**: Structured logging with `tracing` to facilitate *debugging*.
-   🐳 **Docker Support**: Ready to be *containerized* with an efficient `Dockerfile`.
-   🧪 **API Testing**: Equipped with Python scripts for endpoint testing as a Postman replacement.

## 📁 Project Structure

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
-   [PostgreSQL](https://www.postgresql.org/download/) (version 14+).
-   [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for migration management.
    ```sh
    cargo install sqlx-cli --no-default-features --features rustls,postgres
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
    DATABASE_URL=postgres://postgres:postgres@localhost:5432/starter_axum

    # Server
    SERVER_HOST=127.0.0.1
    SERVER_PORT=8000

    # JWT Secrets
    JWT_SECRET=super_secure_secret
    JWT_ACCESS_TOKEN_EXPIRES_IN=15m
    JWT_REFRESH_TOKEN_EXPIRES_IN=7d
    ```

3.  **Setup Postgres Database**
    Ensure your Postgres service is running, then create a new database using `psql` or your favorite tool.
    ```sh
    # Example using psql
    psql -U postgres
    CREATE DATABASE starter_axum;
    \q
    ```

4.  **Run Database Migrations**
    This command will create the required tables (`users`, `tokens`, etc.) according to the files in the `migrations/` folder.
    ```sh
    sqlx migrate run
    ```
    > 💡 **Important:** If you change SQL queries within the code, run `cargo sqlx prepare` to update the `.sqlx` cache.

5.  **Run Application**
    ```sh
    cargo run
    ```
    🎉 Your server is now running at `http://127.0.0.1:8000`.

---

### 🐳 Method 2: Using Docker

If you do not want to install Rust or Postgres locally, Docker is the solution.

#### Prerequisites
-   [Docker](https://www.docker.com/get-started) and Docker Desktop/Engine installed and running.

#### Setup Steps
1.  **Clone Repository** (If you haven't already)
    ```sh
    git clone https://github.com/mnabielap/starter-kit-restapi-axum.git
    cd starter-kit-restapi-axum
    ```

2.  **Create `.env.docker` File**
    The application inside Docker will use this file for configuration. Note the `DATABASE_URL` pointing to the database container name.
    ```env
    # .env.docker
    DATABASE_URL=postgres://postgres:postgres@postgres-db:5432/starter_axum
    SERVER_HOST=0.0.0.0
    SERVER_PORT=8000
    JWT_SECRET=super_secure_docker_secret
    JWT_ACCESS_TOKEN_EXPIRES_IN=15m
    JWT_REFRESH_TOKEN_EXPIRES_IN=7d
    ```

3.  **Create Docker Network & Volumes**
    This only needs to be done once. The Network is for communication between containers, and Volumes are to store data persistently.
    ```sh
    docker network create restapi_axum_network
    docker volume create restapi_axum_db_volume
    docker volume create restapi_axum_media_volume
    ```

4.  **Run Postgres Database Container**
    ```sh
    docker run -d \
      --name postgres-db \
      --network restapi_axum_network \
      -p 5433:5432 \
      -e POSTGRES_USER=postgres \
      -e POSTGRES_PASSWORD=postgres \
      -e POSTGRES_DB=starter_axum \
      -v restapi_axum_db_volume:/var/lib/postgresql/data \
      --restart always \
      postgres:16-alpine
    ```

5.  **Run Migrations to Docker Database**
    From your local computer, run migrations targeting the newly created database container.
    ```sh
    sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5433/starter_axum"
    ```

6.  **Build Application Image**
    ```sh
    docker build -t restapi-axum-app .
    ```

7.  **Run Application Container**
    This container will use variables from `.env.docker` that we created.
    ```sh
    docker run -d -p 5005:8000 \
      --name restapi-axum-container \
      --network restapi_axum_network \
      --env-file .env.docker \
      -v restapi_axum_media_volume:/app/uploads \
      --restart always \
      restapi-axum-app
    ```
    🎉 Your server is now running inside Docker and can be accessed at `http://localhost:5005`.

## 🧪 API Testing (Postman Replacement)

The `api_tests/` directory contains simple Python scripts to test each endpoint. This is very useful for quick verification without needing to open Postman.

#### How to Run Tests
Simply run the relevant Python file from the terminal. The script will print the API response output.
```sh
# Example running registration test
python api_tests/A1.auth_register.py

# Example running login test
python api_tests/A2.auth_login.py
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
    docker logs -f restapi-axum-container
    ```
-   🛑 **Stop container**
    ```sh
    docker stop restapi-axum-container
    ```
-   ▶️ **Start an existing container**
    ```sh
    docker start restapi-axum-container
    ```
-   🗑️ **Remove container (after stopping)**
    ```sh
    docker rm restapi-axum-container
    ```
-   🗂️ **List existing volumes**
    ```sh
    docker volume ls
    ```
-   ⚠️ **Remove volume (WARNING: This will permanently delete data!)**
    ```sh
    # Careful when running this command!
    docker volume rm restapi_axum_db_volume
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