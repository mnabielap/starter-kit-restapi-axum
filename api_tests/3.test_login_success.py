import requests
import json

BASE_URL = "http://localhost:8000/v1/auth"
ENDPOINT = "/login"

VALID_CREDENTIALS = {
    "email": "login_user@example.com",
    "password": "password123"
}

def ensure_user_exists():
    """Memastikan user untuk login ada."""
    register_payload = {
        "name": "Login User",
        "email": VALID_CREDENTIALS["email"],
        "password": VALID_CREDENTIALS["password"]
    }
    headers = {'Content-Type': 'application/json'}
    requests.post(f"{BASE_URL}/register", headers=headers, data=json.dumps(register_payload))
    print(f"[*] Memastikan user dengan email '{VALID_CREDENTIALS['email']}' ada...")


def test_successful_login():
    """
    Mengirim request login dengan kredensial yang valid.
    Ekspektasi: Status code 200 (OK) dan mendapatkan user serta token.
    """
    print(f"\n[*] Menguji: POST {BASE_URL}{ENDPOINT}")
    print(f"[*] Payload: {json.dumps({'email': VALID_CREDENTIALS['email'], 'password': '...'})}")
    
    headers = {
        'Content-Type': 'application/json'
    }

    try:
        response = requests.post(f"{BASE_URL}{ENDPOINT}", headers=headers, data=json.dumps(VALID_CREDENTIALS))
        
        print(f"\n[+] Status Code: {response.status_code}")
        print("[+] Response JSON:")
        try:
            print(json.dumps(response.json(), indent=2))
        except json.JSONDecodeError:
            print(response.text)

    except requests.exceptions.ConnectionError as e:
        print(f"\n[!] Gagal terhubung ke server: {e}")

if __name__ == "__main__":
    ensure_user_exists()
    test_successful_login()