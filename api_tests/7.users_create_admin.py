import time
import sys
import os
sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from utils import send_and_print, save_config, load_config, BASE_URL

access_token = load_config("access_token")

if not access_token:
    print("Missing access token. Login as Admin first.")
else:
    timestamp = int(time.time())
    
    headers = {
        "Authorization": f"Bearer {access_token}"
    }

    payload = {
        "name": f"New Admin {timestamp}",
        "email": f"admin_{timestamp}@example.com",
        "password": "password123",
        "role": "admin"  # Can be "user" or "admin"
    }

    response = send_and_print(
        f"{BASE_URL}/users",
        method="POST",
        headers=headers,
        output_file=f"{os.path.splitext(os.path.basename(__file__))[0]}.json",
        body=payload
    )

    if response.status_code == 201:
        data = response.json()
        # Save this new user ID to target it with update/delete scripts
        save_config("target_user_id", data["id"])
        print("\n[SUCCESS] New user created. ID saved as 'target_user_id'.")