import requests
from helpers import get_user_credentials, print_response, BASE_URL

def test_logout():
    """
    Menguji fungsionalitas logout.
    Ekspektasi: Status 204 (No Content) dan refresh token menjadi tidak valid.
    """
    print("--- Menguji Logout ---")
    creds = get_user_credentials()
    if not creds:
        return

    payload = {"refreshToken": creds["refresh_token"]}
    
    # 1. Lakukan logout
    print("\n[*] Mencoba logout...")
    response = requests.post(f"{BASE_URL}/auth/logout", json=payload)
    print_response(response)
    
    # 2. Verifikasi: Coba gunakan refresh token yang sama lagi
    print("\n[*] Mencoba menggunakan refresh token yang sudah di-logout...")
    verify_response = requests.post(f"{BASE_URL}/auth/refresh-tokens", json=payload)
    print_response(verify_response)
    print("\n[v] Tes Selesai. Ekspektasi: Logout berhasil (204) dan verifikasi gagal (401).")

if __name__ == "__main__":
    test_logout()