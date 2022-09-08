from functools import wraps
from Utils.JSON import JSON

def login_required(func):
    @wraps(func)
    def decorator(*args, **kwargs):
        res = func(*args, **kwargs)
    return decorator
