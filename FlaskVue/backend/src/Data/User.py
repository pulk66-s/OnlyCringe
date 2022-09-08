from Utils.Encrypt import Encrypt

class User:

    def __init__(self, uuid="", name="", password=""):
        self.uuid = uuid
        self.name = name
        self.password = Encrypt.encrypt_password(password, name)

    def __str__(self):
        txt = f"Uuid: {self.uuid}, Name: {self.name}, Password: {self.password}"
        return txt

    def __dict__(self):
        return {
            "uuid": self.uuid,
            "name": self.name,
            "password": self.password
        }