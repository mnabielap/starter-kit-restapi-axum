# --- STAGE 1: Builder ---
# Using Rust version 1.89
FROM rust:1.89-bullseye as builder

# Install dependencies required by `sqlx` for compilation
RUN apt-get update && apt-get install -y libpq-dev pkg-config

WORKDIR /usr/src/app

# Copy important files for dependency caching
COPY Cargo.toml ./Cargo.toml

# Create a dummy project to cache the dependency layer
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy binary, we only need the dependencies
RUN rm -f target/release/deps/starter_kit_restapi_axum*

# Copy the application source code and prepared sqlx data
COPY src ./src
COPY .sqlx ./.sqlx

# Fully build the application in release mode
# `touch` ensures the timestamp changes so the build will run
# Now the build will use data from .sqlx and no DB connection is required
RUN touch src/main.rs && cargo build --release


# --- STAGE 2: Final Image ---
# Use a lightweight base image
FROM debian:bullseye-slim

# Create a non-root user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory inside container
WORKDIR /app

# Copy the compiled binary from the stage builder
COPY --from=builder /usr/src/app/target/release/starter-kit-restapi-axum .

# Create a directory for media/uploads (as requested for volumes)
RUN mkdir -p /app/uploads && chown -R appuser:appuser /app

# Change ownership of application files to non-root user
RUN chown appuser:appuser ./starter-kit-restapi-axum

# Change user to non-root
USER appuser

# Expose the ports used by the application inside the container (as per .env)
EXPOSE 8000

# Run command to start app
CMD './starter-kit-restapi-axum'