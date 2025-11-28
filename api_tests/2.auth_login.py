from utils import send_and_print, save_config, load_config, BASE_URL

# Load the email registered previously, or use a default
email = load_config("last_registered_email") or "admin@example.com"
password = "password123"

print(f"--- Attempting to login as: {email} ---")

payload = {
    "email": email,
    "password": password
}

response = send_and_print(
    f"{BASE_URL}/auth/login",
    method="POST",
    body=payload
)

if response.status_code == 200:
    data = response.json()
    save_config("access_token", data["tokens"]["accessToken"]["token"])
    save_config("refresh_token", data["tokens"]["refreshToken"]["token"])
    save_config("current_user_id", data["user"]["id"])
    print("\n[SUCCESS] Logged in and tokens updated in secrets.json")