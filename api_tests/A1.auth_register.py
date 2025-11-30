import time
import sys
import os
sys.path.append(os.path.abspath(os.path.dirname(__file__)))
from utils import send_and_print, save_config, BASE_URL

# Generate a unique email to ensure registration always works
timestamp = int(time.time())
email = f"user_{timestamp}@example.com"
password = "password123"

print(f"--- Attempting to register: {email} ---")

payload = {
    "name": f"User {timestamp}",
    "email": email,
    "password": password
}

response = send_and_print(
    f"{BASE_URL}/auth/register",
    method="POST",
    output_file=f"{os.path.splitext(os.path.basename(__file__))[0]}.json",
    body=payload
)

if response.status_code == 201:
    data = response.json()
    # Save tokens for subsequent requests
    save_config("access_token", data["tokens"]["accessToken"]["token"])
    save_config("refresh_token", data["tokens"]["refreshToken"]["token"])
    # Save user ID for profile tests
    save_config("current_user_id", data["user"]["id"])
    
    # Save credentials to config just for the login script to use later
    save_config("last_registered_email", email)
    print("\n[SUCCESS] User registered and tokens saved to secrets.json")