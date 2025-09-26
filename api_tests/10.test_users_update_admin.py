import requests
from helpers import get_admin_credentials, get_user_credentials, print_response, BASE_URL

def test_update_user_as_admin():
    """
    Menguji pembaruan data pengguna lain oleh admin.
    Ekspektasi: Status 200 dan mendapatkan data pengguna yang sudah diperbarui.
    """
    print("--- Menguji Update User (sebagai Admin) ---")
    admin_creds = get_admin_credentials()
    target_user_creds = get_user_credentials()
    
    if not admin_creds or not target_user_creds:
        return

    headers = {"Authorization": f"Bearer {admin_creds['access_token']}"}
    target_user_id = target_user_creds['user_id']
    update_payload = {"name": "Nama Telah Diperbarui oleh Admin"}
    
    print(f"\n[*] Mencoba memperbarui pengguna dengan ID: {target_user_id}")
    response = requests.patch(f"{BASE_URL}/users/{target_user_id}", headers=headers, json=update_payload)
    print_response(response)
    print("\n[v] Tes Selesai. Ekspektasi: Data pengguna berhasil diperbarui (200).")

if __name__ == "__main__":
    test_update_user_as_admin()