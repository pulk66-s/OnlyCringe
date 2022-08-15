import requests

class API:

    def __init__(self, url):
        self.base_url = url

    def get(self, url):
        return requests.get(self.base_url + url)

    def post(self, url, data):
        return requests.post(self.base_url + url, json=data)

    def put(self, url, data):
        return requests.put(self.base_url + url, json=data)

    def delete(self, url):
        return requests.delete(self.base_url + url)
