from utils import send_and_print, load_config, BASE_URL

access_token = load_config("access_token")

if not access_token:
    print("Missing access token. Login first.")
else:
    headers = {
        "Authorization": f"Bearer {access_token}"
    }

    # Query params: page=1, limit=5
    send_and_print(
        f"{BASE_URL}/users?page=1&limit=5",
        method="GET",
        headers=headers
    )