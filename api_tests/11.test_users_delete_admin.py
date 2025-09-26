import requests
from helpers import get_admin_credentials, get_user_credentials, print_response, BASE_URL

def test_delete_user_as_admin():
    """
    Menguji penghapusan pengguna oleh admin.
    Ekspektasi: Status 204 dan pengguna tidak lagi bisa diakses.
    """
    print("--- Menguji Delete User (sebagai Admin) ---")
    admin_creds = get_admin_credentials()
    target_user_creds = get_user_credentials()
    
    if not admin_creds or not target_user_creds:
        return

    headers = {"Authorization": f"Bearer {admin_creds['access_token']}"}
    target_user_id = target_user_creds['user_id']
    
    # 1. Hapus pengguna
    print(f"\n[*] Mencoba menghapus pengguna dengan ID: {target_user_id}")
    response = requests.delete(f"{BASE_URL}/users/{target_user_id}", headers=headers)
    print_response(response)
    
    # 2. Verifikasi: Coba akses pengguna yang sudah dihapus
    print(f"\n[*] Verifikasi: Mencoba mengambil data pengguna yang sudah dihapus...")
    verify_response = requests.get(f"{BASE_URL}/users/{target_user_id}", headers=headers)
    print_response(verify_response)
    print("\n[v] Tes Selesai. Ekspektasi: Penghapusan berhasil (204) dan verifikasi gagal (404).")

if __name__ == "__main__":
    test_delete_user_as_admin()