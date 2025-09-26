import requests
import json
import uuid

BASE_URL = "http://localhost:8000/v1/auth"
ENDPOINT = "/register"

def test_successful_registration():
    """
    Mengirim request registrasi dengan data valid dan email unik.
    Ekspektasi: Status code 201 (Created) dan mendapatkan user serta token.
    """
    unique_email = f"testuser_{uuid.uuid4()}@example.com"
    
    payload = {
        "name": "Test User",
        "email": unique_email,
        "password": "password123"
    }
    
    headers = {
        'Content-Type': 'application/json'
    }
    
    print(f"[*] Menguji: POST {BASE_URL}{ENDPOINT}")
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
        print("[!] Pastikan server Rust Axum Anda sudah berjalan di http://localhost:8000")

if __name__ == "__main__":
    test_successful_registration()