from base64 import b64encode
import os

class ProfileImg:

    def __init__(self):
        self.global_path = "res"
        self.profile_path = self.global_path + "/profile"

    def get(self, uuid):
        try:
            file = open(self.profile_path + "/" + uuid + ".png", "rb")
            res = file.read()
            return b64encode(res), 200
        except Exception as e:
            return "NOT FOUND", 204
    
    def post(self, uuid, data):
        file = open(self.profile_path + "/" + uuid + ".png", "wb")
        file.write(data)
        file.close()
        return "OK"

    def delete(self, uuid):
        try:
            os.remove(self.profile_path + "/" + uuid + ".png")
            return "OK"
        except Exception as e:
            return "NOT FOUND", 204