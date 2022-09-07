from flask import jsonify

class JSON:

    def parse(data):
        if isinstance(data, (dict, list, tuple, str, int, float, bool)):
            return jsonify(data)
        return data.__dict__()