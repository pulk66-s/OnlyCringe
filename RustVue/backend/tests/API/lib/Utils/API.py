import requests

class API:

    def __init__(self, host="localhost", port="8080"):
        self.host = host
        self.port = port
        self.base_url = f"http://{self.host}:{self.port}"

    def get(self, path):
        return requests.get(self.base_url + path)

    def post(self, path, data):
        return requests.post(self.base_url + path, json=data)

    def put(self, path, data):
        return requests.put(self.base_url + path, json=data)

    def delete(self, path):
        return requests.delete(self.base_url + path)
