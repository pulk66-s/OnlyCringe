class User:

    def __init__(self, uuid="", name="", password=""):
        self.uuid = uuid
        self.name = name
        self.password = password

    def __str__(self):
        txt = f"Name: {self.name}, Password: {self.password}"
        return txt

    def __dict__(self):
        return {
            "uuid": self.uuid,
            "name": self.name,
            "password": self.password
        }