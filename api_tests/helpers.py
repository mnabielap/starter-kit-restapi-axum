import requests
import json
import uuid

BASE_URL = "http://localhost:8000/v1"

# --- Kredensial untuk Pengujian ---
# Pastikan pengguna ini sudah ada dan perannya adalah 'admin' di database
ADMIN_EMAIL = "admin@example.com"
ADMIN_PASSWORD = "adminpassword123"

def get_admin_credentials():
    """Login sebagai admin dan mengembalikan token serta ID."""
    print("[*] Melakukan login sebagai ADMIN...")
    payload = {"email": ADMIN_EMAIL, "password": ADMIN_PASSWORD}
    try:
        response = requests.post(f"{BASE_URL}/auth/login", json=payload)
        if response.status_code == 200:
            data = response.json()
            print("[+] Login admin berhasil.")
            return {
                "access_token": data["tokens"]["accessToken"]["token"],
                "user_id": data["user"]["id"]
            }
        else:
            print(f"[!] Gagal login sebagai admin. Status: {response.status_code}, Pesan: {response.text}")
            return None
    except requests.exceptions.ConnectionError:
        print("[!] Gagal terhubung ke server. Pastikan server Rust berjalan.")
        return None

def get_user_credentials():
    """Mendaftarkan pengguna baru, login, dan mengembalikan token serta ID."""
    unique_email = f"testuser_{uuid.uuid4()}@example.com"
    print(f"[*] Membuat dan login sebagai pengguna baru: {unique_email}")
    
    register_payload = {"name": "Test User", "email": unique_email, "password": "password123"}
    login_payload = {"email": unique_email, "password": "password123"}
    
    try:
        # Register
        requests.post(f"{BASE_URL}/auth/register", json=register_payload)
        
        # Login
        response = requests.post(f"{BASE_URL}/auth/login", json=login_payload)
        
        if response.status_code == 200:
            data = response.json()
            print("[+] Pembuatan & login pengguna baru berhasil.")
            return {
                "access_token": data["tokens"]["accessToken"]["token"],
                "refresh_token": data["tokens"]["refreshToken"]["token"],
                "user_id": data["user"]["id"]
            }
        else:
            print(f"[!] Gagal login setelah registrasi. Status: {response.status_code}, Pesan: {response.text}")
            return None
            
    except requests.exceptions.ConnectionError:
        print("[!] Gagal terhubung ke server. Pastikan server Rust berjalan.")
        return None

def print_response(response):
    """Mencetak respons HTTP dengan format yang rapi."""
    print(f"\n[+] Status Code: {response.status_code}")
    print("[+] Response Body:")
    try:
        print(json.dumps(response.json(), indent=2))
    except json.JSONDecodeError:
        # Cetak teks kosong jika tidak ada body (misal, status 204)
        if response.text:
            print(response.text)
        else:
            print("(No content)")