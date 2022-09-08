from dotenv import load_dotenv
import os
import hashlib

class Encrypt:

    def encrypt_password(password, salt=None):
        load_dotenv()
        env_salt = os.getenv("ENCRYPT_SALT")
        salt = env_salt if salt is None else salt + env_salt
        combined = password + salt
        return hashlib.sha512(bytes(combined, encoding="utf-8")).hexdigest()

    def encrypt_user(user):
        load_dotenv()
        salt = os.getenv("ENCRYPT_SALT")
        data = user.uuid + user.name + user.password + salt;
        return hashlib.sha512(bytes(data, encoding="utf-8")).hexdigest()