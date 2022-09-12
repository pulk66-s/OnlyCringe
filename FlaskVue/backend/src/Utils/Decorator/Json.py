from functools import wraps
from Utils.JSON import JSON

def json_response(func):
    @wraps(func)
    def decorator(*args, **kwargs):
        try:
            res = func()
        except Exception as e:
            e = str(e).split(", ")
            return e[0], int(e[1])
        code = 200
        if len(res) == 2 and isinstance(res[1], int):
            res, code = res
        return JSON.parse(res), code
    return decorator
