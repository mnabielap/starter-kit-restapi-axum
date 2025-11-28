import sys
import os
sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from utils import send_and_print, save_config, load_config, BASE_URL

refresh_token = load_config("refresh_token")

if not refresh_token:
    print("No refresh token found. Please register or login first.")
else:
    payload = {
        "refreshToken": refresh_token
    }

    response = send_and_print(
        f"{BASE_URL}/auth/refresh-tokens",
        method="POST",
        output_file=f"{os.path.splitext(os.path.basename(__file__))[0]}.json",
        body=payload
    )

    if response.status_code == 200:
        data = response.json()
        save_config("access_token", data["accessToken"]["token"])
        save_config("refresh_token", data["refreshToken"]["token"])
        print("\n[SUCCESS] Tokens refreshed and saved.")