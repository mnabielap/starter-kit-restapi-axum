# 🚀 Starter Kit REST API dengan Axum, SQLx, dan Postgres

<div align="center">

![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)![SQLx](https://img.shields.io/badge/sqlx-0.7-green.svg)![PostgreSQL](https://img.shields.io/badge/postgres-16-blue.svg)![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)

</div>

Selamat datang di Starter Kit REST API berbasis **Axum**, sebuah fondasi yang kuat, modern, dan siap produksi untuk membangun layanan backend dengan Rust. Proyek ini dirancang dengan arsitektur berlapis (*Usecase*, *Repository*, *Handler*) untuk memastikan kode yang terorganisir, dapat diuji, dan mudah dikelola.

## ✨ Fitur Utama

-   ✅ **Framework Web Modern**: Dibangun di atas [Axum](https://github.com/tokio-rs/axum) yang cepat dan ergonomis.
-   🔒 **Autentikasi JWT**: Implementasi lengkap untuk *Register*, *Login*, *Logout*, dan *Refresh Token*.
-   🔐 **Keamanan**: *Password Hashing* menggunakan `bcrypt`.
-   🏢 **Arsitektur Berlapis**: Pemisahan yang jelas antara logika bisnis (*Usecase*), akses data (*Repository*), dan rute API (*Handler*).
-   👤 **Manajemen Pengguna & Peran**: CRUD untuk pengguna dengan sistem peran (*Admin* & *User*).
-   🛡️ **Middleware & Rute Terproteksi**: Contoh penggunaan *middleware* untuk otentikasi dan otorisasi berbasis peran.
-   🐘 **Database Postgres**: Menggunakan [SQLx](https://github.com/launchbadge/sqlx) dengan *compile-time checked queries* untuk keamanan tipe.
-   📚 **Dokumentasi API Otomatis**: Swagger UI terintegrasi dengan [Utoipa](https://github.com/juhaku/utoipa).
-   ⚙️ **Konfigurasi Fleksibel**: Manajemen konfigurasi melalui file `.env` dan *environment variables*.
-   📝 **Logging**: Logging terstruktur dengan `tracing` untuk memudahkan *debugging*.
-   🐳 **Dukungan Docker**: Siap untuk di-*containerize* dengan `Dockerfile` yang efisien.
-   🧪 **Testing API**: Dilengkapi dengan skrip Python untuk pengujian endpoint sebagai pengganti Postman.

## 📁 Struktur Proyek

```
.
├── api_tests/              # Skrip pengujian API (pengganti Postman)
├── migrations/             # File migrasi database SQLx
├── src/
│   ├── config/             # Modul konfigurasi aplikasi
│   ├── domain/             # Model data dan struct utama
│   ├── error/              # Penanganan error kustom
│   ├── handler/            # Logika untuk menangani request HTTP
│   ├── middleware/         # Middleware otentikasi & otorisasi
│   ├── repository/         # Logika akses data ke database
│   ├── routes/             # Definisi rute API dan wiring
│   └── usecase/            # Logika bisnis inti aplikasi
├── .env                    # File konfigurasi untuk development lokal (TIDAK di-commit)
├── .env.docker             # File konfigurasi untuk Docker
├── .sqlx/                  # Cache metadata SQLx untuk mode offline
├── Cargo.toml              # Dependensi proyek
├── Dockerfile              # Resep untuk membangun image Docker
└── entrypoint.sh           # Skrip entrypoint untuk kontainer Docker
```

## 🚀 Memulai Proyek

Kami merekomendasikan untuk menjalankan proyek ini di lingkungan lokal terlebih dahulu untuk memahami alurnya. Jika Anda mengalami kesulitan dengan setup lokal, Docker adalah alternatif yang sangat baik.

### 👩‍💻 Metode 1: Development Lokal (Direkomendasikan)

#### Persyaratan
-   [Rust](https://www.rust-lang.org/tools/install) (versi 1.89+).
-   [PostgreSQL](https://www.postgresql.org/download/) (versi 14+).
-   [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) untuk manajemen migrasi.
    ```sh
    cargo install sqlx-cli --no-default-features --features rustls,postgres
    ```

#### Langkah-langkah Setup
1.  **Clone Repositori**
    ```sh
    git clone https://github.com/mnabielap/starter-kit-restapi-axum.git
    cd starter-kit-restapi-axum
    ```

2.  **Buat File `.env`**
    Buat file bernama `.env` di root proyek dan salin konten berikut. Sesuaikan jika perlu.
    ```env
    # .env
    DATABASE_URL=postgres://postgres:postgres@localhost:5432/starter_axum

    # Server
    SERVER_HOST=127.0.0.1
    SERVER_PORT=8000

    # JWT Secrets
    JWT_SECRET=rahasia_super_aman
    JWT_ACCESS_TOKEN_EXPIRES_IN=15m
    JWT_REFRESH_TOKEN_EXPIRES_IN=7d
    ```

3.  **Setup Database Postgres**
    Pastikan layanan Postgres Anda berjalan, lalu buat database baru menggunakan `psql` atau tool favorit Anda.
    ```sh
    # Contoh menggunakan psql
    psql -U postgres
    CREATE DATABASE starter_axum;
    \q
    ```

4.  **Jalankan Migrasi Database**
    Perintah ini akan membuat tabel-tabel yang dibutuhkan (`users`, `tokens`, dll.) sesuai dengan file di folder `migrations/`.
    ```sh
    sqlx migrate run
    ```
    > 💡 **Penting:** Jika Anda mengubah query SQL di dalam kode, jalankan `cargo sqlx prepare` untuk mengupdate cache `.sqlx`.

5.  **Jalankan Aplikasi**
    ```sh
    cargo run
    ```
    🎉 Server Anda sekarang berjalan di `http://127.0.0.1:8000`.

---

### 🐳 Metode 2: Menggunakan Docker

Jika Anda tidak ingin menginstal Rust atau Postgres secara lokal, Docker adalah solusinya.

#### Persyaratan
-   [Docker](https://www.docker.com/get-started) dan Docker Desktop/Engine terinstal dan berjalan.

#### Langkah-langkah Setup
1.  **Clone Repositori** (Jika belum)
    ```sh
    git clone https://github.com/username/starter-kit-restapi-axum.git
    cd starter-kit-restapi-axum
    ```

2.  **Buat File `.env.docker`**
    Aplikasi di dalam Docker akan menggunakan file ini untuk konfigurasi. Perhatikan `DATABASE_URL` yang menunjuk ke nama kontainer database.
    ```env
    # .env.docker
    DATABASE_URL=postgres://postgres:postgres@postgres-db:5432/starter_axum
    SERVER_HOST=0.0.0.0
    SERVER_PORT=8000
    JWT_SECRET=rahasia_super_aman_docker
    JWT_ACCESS_TOKEN_EXPIRES_IN=15m
    JWT_REFRESH_TOKEN_EXPIRES_IN=7d
    ```

3.  **Buat Network & Volume Docker**
    Ini hanya perlu dilakukan sekali. Network untuk komunikasi antar kontainer, dan Volume untuk menyimpan data secara persisten.
    ```sh
    docker network create restapi_axum_network
    docker volume create restapi_axum_db_volume
    docker volume create restapi_axum_media_volume
    ```

4.  **Jalankan Kontainer Database Postgres**
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

5.  **Jalankan Migrasi ke Database Docker**
    Dari komputer lokal Anda, jalankan migrasi yang menargetkan kontainer database yang baru saja dibuat.
    ```sh
    sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5433/starter_axum"
    ```

6.  **Build Image Aplikasi**
    ```sh
    docker build -t velzon-axum-app .
    ```

7.  **Jalankan Kontainer Aplikasi**
    Kontainer ini akan menggunakan variabel dari `.env.docker` yang kita buat.
    ```sh
    docker run -d -p 5005:8000 \
      --name velzon-axum-container \
      --network restapi_axum_network \
      --env-file .env.docker \
      -v restapi_axum_media_volume:/app/uploads \
      --restart always \
      velzon-axum-app
    ```
    🎉 Server Anda sekarang berjalan di dalam Docker dan dapat diakses di `http://localhost:5005`.

## 🧪 Pengujian API (Pengganti Postman)

Direktori `api_tests/` berisi skrip Python sederhana untuk menguji setiap endpoint. Ini sangat berguna untuk verifikasi cepat tanpa perlu membuka Postman.

#### Persyaratan
-   [Python 3](https://www.python.org/downloads/)
-   Library `requests`.
    ```sh
    pip install requests
    ```

#### Cara Menjalankan Tes
Cukup jalankan file Python yang relevan dari terminal. Skrip akan mencetak output dari response API.
```sh
# Contoh menjalankan tes registrasi sukses
python api_tests/1.test_register_success.py

# Contoh menjalankan tes login
python api_tests/3.test_login_success.py
```
> 💡 Skrip ini secara default menargetkan `http://localhost:8000`. Jika Anda menggunakan Docker, ubah URL di dalam skrip menjadi `http://localhost:5005`.

## 📚 Dokumentasi API (Swagger UI)

Proyek ini secara otomatis menghasilkan dokumentasi OpenAPI. Anda bisa mengaksesnya melalui browser untuk melihat semua endpoint yang tersedia dan mencobanya secara langsung.

-   **Development Lokal**: [http://localhost:8000/swagger-ui](http://localhost:8000/swagger-ui)
-   **Docker**: [http://localhost:5005/swagger-ui](http://localhost:5005/swagger-ui)

## ⚙️ Manajemen Kontainer Docker (Cheat Sheet)

Berikut adalah beberapa perintah Docker yang berguna untuk mengelola kontainer Anda.

-   🪵 **Melihat log dari kontainer yang berjalan**
    ```sh
    docker logs -f velzon-axum-container
    ```
-   🛑 **Menghentikan kontainer**
    ```sh
    docker stop velzon-axum-container
    ```
-   ▶️ **Memulai kembali kontainer yang sudah ada**
    ```sh
    docker start velzon-axum-container
    ```
-   🗑️ **Menghapus kontainer (setelah dihentikan)**
    ```sh
    docker rm velzon-axum-container
    ```
-   🗂️ **Melihat daftar volume yang ada**
    ```sh
    docker volume ls
    ```
-   ⚠️ **Menghapus volume (PERHATIAN: Ini akan menghapus data secara permanen!)**
    ```sh
    # Hati-hati saat menjalankan perintah ini!
    docker volume rm restapi_axum_db_volume
    ```

## 🛠️ Teknologi & Library Utama

-   **Framework**: [Axum](https://github.com/tokio-rs/axum)
-   **Async Runtime**: [Tokio](https://tokio.rs/)
-   **Database ORM/Toolkit**: [SQLx](https://github.com/launchbadge/sqlx)
-   **Serialisasi/Deserialisasi**: [Serde](https://serde.rs/)
-   **Dokumentasi API**: [Utoipa](https://github.com/juhaku/utoipa)
-   **Konfigurasi**: [config-rs](https://github.com/mehcode/config-rs), [dotenvy](https://github.com/allan2/dotenvy)
-   **Validasi**: [validator](https://github.com/Keats/validator)
-   **JWT**: [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
-   **Logging**: [tracing](https://github.com/tokio-rs/tracing)

## 📄 Lisensi

Proyek ini dilisensikan di bawah [Lisensi MIT](LICENSE).