import requests
from helpers import get_user_credentials, print_response, BASE_URL

def test_refresh_token():
    """
    Menguji fungsionalitas refresh token.
    Ekspektasi: Status 200 dan mendapatkan sepasang token baru.
    """
    print("--- Menguji Refresh Token ---")
    creds = get_user_credentials()
    if not creds:
        return

    payload = {"refreshToken": creds["refresh_token"]}
    
    print("\n[*] Mencoba merefresh token...")
    response = requests.post(f"{BASE_URL}/auth/refresh-tokens", json=payload)
    print_response(response)
    print("\n[v] Tes Selesai. Ekspektasi: Mendapatkan token baru (200).")

if __name__ == "__main__":
    test_refresh_token()