import requests
from helpers import get_admin_credentials, print_response, BASE_URL

def test_get_all_users_as_admin():
    """
    Menguji pengambilan semua data pengguna oleh admin.
    Ekspektasi: Status 200 dan mendapatkan daftar pengguna yang dipaginasi.
    """
    print("--- Menguji Get All Users (sebagai Admin) ---")
    admin_creds = get_admin_credentials()
    if not admin_creds:
        return

    headers = {"Authorization": f"Bearer {admin_creds['access_token']}"}
    
    print("\n[*] Mencoba mengambil semua pengguna...")
    response = requests.get(f"{BASE_URL}/users", headers=headers)
    print_response(response)
    print("\n[v] Tes Selesai. Ekspektasi: Mendapatkan daftar pengguna (200).")

if __name__ == "__main__":
    test_get_all_users_as_admin()