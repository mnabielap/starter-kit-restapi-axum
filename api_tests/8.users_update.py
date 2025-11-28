from utils import send_and_print, load_config, BASE_URL

access_token = load_config("access_token")
target_id = load_config("target_user_id")

if not access_token or not target_id:
    print("Missing token or target_user_id. Run users_create_admin.py first.")
else:
    headers = {
        "Authorization": f"Bearer {access_token}"
    }

    payload = {
        "name": "Updated Name via Python",
        # "email": "updated@example.com" # Optional
    }

    send_and_print(
        f"{BASE_URL}/users/{target_id}",
        method="PATCH",
        headers=headers,
        body=payload
    )