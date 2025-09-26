import requests
from helpers import get_user_credentials, print_response, BASE_URL

def test_get_user_by_id():
    """
    Menguji pengambilan data pengguna berdasarkan ID.
    Ekspektasi: Status 200 dan mendapatkan data pengguna sendiri.
    """
    print("--- Menguji Get User by ID ---")
    creds = get_user_credentials()
    if not creds:
        return

    headers = {"Authorization": f"Bearer {creds['access_token']}"}
    user_id = creds['user_id']
    
    print(f"\n[*] Mencoba mengambil data pengguna dengan ID: {user_id}")
    response = requests.get(f"{BASE_URL}/users/{user_id}", headers=headers)
    print_response(response)
    print("\n[v] Tes Selesai. Ekspektasi: Mendapatkan data pengguna (200).")

if __name__ == "__main__":
    test_get_user_by_id()