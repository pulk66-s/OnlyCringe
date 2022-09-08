from functools import wraps
from Utils.JSON import JSON

def json_response(func):
    @wraps(func)
    def decorator(*args, **kwargs):
        res = func()
        code = 200
        if len(res) == 2 and isinstance(res[1], int):
            res, code = res
        return JSON.parse(res), code
    return decorator
