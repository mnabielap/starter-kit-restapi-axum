import requests
import uuid
from helpers import get_admin_credentials, print_response, BASE_URL

def test_create_user_as_admin():
    """
    Menguji pembuatan pengguna baru oleh admin.
    Ekspektasi: Status 201 (Created) dan data pengguna baru dikembalikan.
    """
    print("--- Menguji Create User (sebagai Admin) ---")
    admin_creds = get_admin_credentials()
    if not admin_creds:
        return

    headers = {"Authorization": f"Bearer {admin_creds['access_token']}"}
    new_user_payload = {
        "name": "Created by Admin",
        "email": f"new_user_{uuid.uuid4()}@example.com",
        "password": "password123",
        "role": "user"
    }
    
    print("\n[*] Mencoba membuat pengguna baru...")
    response = requests.post(f"{BASE_URL}/users", headers=headers, json=new_user_payload)
    print_response(response)
    print("\n[v] Tes Selesai. Ekspektasi: Pengguna berhasil dibuat (201).")

if __name__ == "__main__":
    test_create_user_as_admin()