import sys
import os
sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from utils import send_and_print, load_config, BASE_URL

access_token = load_config("access_token")
target_id = load_config("target_user_id")

if not access_token or not target_id:
    print("Missing token or target_user_id. Run users_create_admin.py first.")
else:
    headers = {
        "Authorization": f"Bearer {access_token}"
    }

    send_and_print(
        f"{BASE_URL}/users/{target_id}",
        method="DELETE",
        output_file=f"{os.path.splitext(os.path.basename(__file__))[0]}.json",
        headers=headers
    )