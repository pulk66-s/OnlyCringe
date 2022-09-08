from functools import wraps
from Utils.Encrypt import Encrypt

def auth_token_gen(func):
    @wraps(func)
    def decorator(*args, **kwargs):
        res = func(*args, **kwargs)
        code = 200
        if len(res) == 2:
            res, code = res
        encrypted = Encrypt.encrypt_user(res)
        return {
            "user": res,
            "token": encrypted
        }, code
    return decorator