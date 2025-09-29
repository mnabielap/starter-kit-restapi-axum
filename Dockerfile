# --- STAGE 1: Builder ---
# Menggunakan versi Rust 1.89
FROM rust:1.89-bullseye as builder

# Instalasi dependensi yang dibutuhkan oleh `sqlx` untuk kompilasi
RUN apt-get update && apt-get install -y libpq-dev pkg-config

WORKDIR /usr/src/app

# Salin file-file penting untuk caching dependensi
COPY Cargo.toml ./Cargo.toml

# Buat dummy project untuk men-cache layer dependensi
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Hapus dummy binary, kita hanya butuh dependensinya
RUN rm -f target/release/deps/starter_kit_restapi_axum*

# Salin source code aplikasi dan data sqlx yang sudah di-prepare
COPY src ./src
COPY .sqlx ./.sqlx

# Build aplikasi secara penuh dalam mode release
# `touch` memastikan timestamp berubah sehingga build akan berjalan
# Sekarang build akan menggunakan data dari .sqlx dan tidak perlu koneksi DB
RUN touch src/main.rs && cargo build --release


# --- STAGE 2: Final Image ---
# Gunakan base image yang ringan
FROM debian:bullseye-slim

# Buat user non-root untuk keamanan
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory di dalam kontainer
WORKDIR /app

# Salin binary yang sudah di-compile dari stage builder
COPY --from=builder /usr/src/app/target/release/starter-kit-restapi-axum .

# Buat direktori untuk media/uploads (seperti yang diminta untuk volume)
RUN mkdir -p /app/uploads && chown -R appuser:appuser /app

# Ganti kepemilikan file aplikasi ke user non-root
RUN chown appuser:appuser ./starter-kit-restapi-axum

# Ganti user ke non-root
USER appuser

# Expose port yang digunakan oleh aplikasi di dalam kontainer (sesuai .env)
EXPOSE 8000

# Run command to start app
CMD './starter-kit-restapi-axum'