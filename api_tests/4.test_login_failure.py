import requests
import json

BASE_URL = "http://localhost:8000/v1/auth"
ENDPOINT = "/login"

INVALID_CREDENTIALS = {
    "email": "login_user@example.com",
    "password": "wrongpassword"
}

def test_failed_login():
    """
    Mengirim request login dengan password yang salah.
    Ekspektasi: Status code 401 (Unauthorized) dan pesan error.
    """
    print(f"[*] Menguji: POST {BASE_URL}{ENDPOINT} (dengan password salah)")
    print(f"[*] Payload: {json.dumps(INVALID_CREDENTIALS)}")
    
    headers = {
        'Content-Type': 'application/json'
    }

    try:
        response = requests.post(f"{BASE_URL}{ENDPOINT}", headers=headers, data=json.dumps(INVALID_CREDENTIALS))
        
        print(f"\n[+] Status Code: {response.status_code}")
        print("[+] Response JSON:")
        try:
            print(json.dumps(response.json(), indent=2))
        except json.JSONDecodeError:
            print(response.text)
            
    except requests.exceptions.ConnectionError as e:
        print(f"\n[!] Gagal terhubung ke server: {e}")


if __name__ == "__main__":
    test_failed_login()