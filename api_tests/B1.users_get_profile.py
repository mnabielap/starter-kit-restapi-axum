import sys
import os
sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from utils import send_and_print, load_config, BASE_URL

access_token = load_config("access_token")
user_id = load_config("current_user_id")

if not access_token or not user_id:
    print("Missing access token or user ID. Login first.")
else:
    headers = {
        "Authorization": f"Bearer {access_token}"
    }

    send_and_print(
        f"{BASE_URL}/users/{user_id}",
        method="GET",
        output_file=f"{os.path.splitext(os.path.basename(__file__))[0]}.json",
        headers=headers
    )