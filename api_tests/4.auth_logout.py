import sys
import os
sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from utils import send_and_print, load_config, BASE_URL

refresh_token = load_config("refresh_token")

if not refresh_token:
    print("No refresh token found.")
else:
    payload = {
        "refreshToken": refresh_token
    }

    send_and_print(
        f"{BASE_URL}/auth/logout",
        method="POST",
        output_file=f"{os.path.splitext(os.path.basename(__file__))[0]}.json",
        body=payload
    )