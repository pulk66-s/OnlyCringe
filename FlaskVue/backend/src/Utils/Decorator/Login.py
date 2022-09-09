from flask import request
from functools import wraps
from Utils.JSON import JSON
from Utils.Encrypt import Encrypt

def login_required(func):
    @wraps(func)
    def decorator(*args, **kwargs):
        if "Authorization" not in request.headers:
            return "Not Authorized", 401
        header = request.headers["Authorization"]
        if Encrypt.decrypt_user(header):
            res = func(*args, **kwargs)
            return res
        return "Not Authorized", 401
    return decorator
