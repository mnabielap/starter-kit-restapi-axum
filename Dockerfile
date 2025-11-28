# --- STAGE 1: Builder ---
FROM rust:1.89-bullseye as builder

# Replace libpq-dev (Postgres) to libsqlite3-dev (SQLite)
RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config

WORKDIR /usr/src/app

# Copy important files for dependency caching
COPY Cargo.toml ./Cargo.toml

# Create a dummy project to cache the dependency layer
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Delete dummy binary
RUN rm -f target/release/deps/starter_kit_restapi_axum*

# Copy the application source code and prepared sqlx data
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations 

# Build the application in release mode (using .sqlx for offline checking)
RUN touch src/main.rs && cargo build --release


# --- STAGE 2: Final Image ---
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# Copy binary
COPY --from=builder /usr/src/app/target/release/starter-kit-restapi-axum .

# Create a directory for uploads AND data (for sqlite files)
# Make sure the permissions belong to appuser
RUN mkdir -p /app/uploads /app/data && chown -R appuser:appuser /app

# Change binary ownership
RUN chown appuser:appuser ./starter-kit-restapi-axum

# Change user
USER appuser

# Set Default Environment Variable (can be overridden at run time)
# We point the DB to the /app/data folder so that it can be mounted as a volume
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8000
ENV DATABASE_URL="sqlite://data/data.db?mode=rwc"

EXPOSE 8000

CMD ["./starter-kit-restapi-axum"]