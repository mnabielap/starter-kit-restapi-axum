import requests
import json

BASE_URL = "http://localhost:8000/v1/auth"
ENDPOINT = "/register"

EXISTING_EMAIL = "duplicate@example.com"

def prepare_duplicate_user():
    """Memastikan user duplikat ada untuk pengujian."""
    payload = {"name": "Duplicate User", "email": EXISTING_EMAIL, "password": "password123"}
    headers = {'Content-Type': 'application/json'}
    requests.post(f"{BASE_URL}{ENDPOINT}", headers=headers, data=json.dumps(payload))
    print(f"[*] Memastikan user dengan email '{EXISTING_EMAIL}' ada...")

def test_duplicate_email_registration():
    """
    Mengirim request registrasi dengan email yang sudah terdaftar.
    Ekspektasi: Status code 400 (Bad Request) dengan pesan error.
    """
    payload = {
        "name": "Another User",
        "email": EXISTING_EMAIL,
        "password": "anotherpassword"
    }
    
    headers = {
        'Content-Type': 'application/json'
    }
    
    print(f"\n[*] Menguji: POST {BASE_URL}{ENDPOINT} (dengan email duplikat)")
    print(f"[*] Payload: {json.dumps(payload, indent=2)}")
    
    try:
        response = requests.post(f"{BASE_URL}{ENDPOINT}", headers=headers, data=json.dumps(payload))
        
        print(f"\n[+] Status Code: {response.status_code}")
        print("[+] Response JSON:")
        try:
            print(json.dumps(response.json(), indent=2))
        except json.JSONDecodeError:
            print(response.text)

    except requests.exceptions.ConnectionError as e:
        print(f"\n[!] Gagal terhubung ke server: {e}")

if __name__ == "__main__":
    prepare_duplicate_user()
    test_duplicate_email_registration()