from dotenv import load_dotenv
import hashlib
import jwt
import os

class Encrypt:

    def encrypt_password(password, salt=None):
        load_dotenv()
        env_salt = os.getenv("ENCRYPT_SALT")
        salt = env_salt if salt is None else salt + env_salt
        combined = password + salt
        return hashlib.sha512(bytes(combined, encoding="utf-8")).hexdigest()

    def encrypt_user(user):
        if isinstance(user, str):
            return user
        load_dotenv()
        salt = os.getenv("ENCRYPT_SALT")
        encoded_jwt = jwt.encode(user.__dict__, salt, algorithm="HS256")
        return encoded_jwt.decode("utf-8")

    def decrypt_user(token):
        load_dotenv()
        salt = os.getenv("ENCRYPT_SALT")
        try:
            jwt.decode(token, salt, algorithm="HS256")
            return True
        except:
            return False