from Utils.Encrypt import Encrypt
from Utils.DB import DB

class User:

    def __fill(self):
        if self.uuid == "" and self.name == "" and self.email == "":
            return
        db = DB()
        req = "select * from User where "
        if self.uuid != "":
            req += f"uuid = '{self.uuid}'"
        elif self.name != "":
            req += f"name = '{self.name}'"
        elif self.email != "":
            req += f"email = '{self.email}'"
        res = db.get(req)
        if len(res) == 0:
            return
        res = res[0]
        self.__init__(
            uuid=res["uuid"] if self.uuid == "" else self.uuid,
            name=res["name"] if self.name == "" else self.name,
            password=res["password"] if self.password == "" else self.password,
            email=res["email"] if self.email == "" else self.email,
            fill=False,
            encrypt_password=False
        )

    def __init__(self, uuid="", name="", password="", email="", fill=True, encrypt_password=True):
        self.uuid = uuid
        self.name = name
        self.password = password
        self.email = email
        if fill is True:
            self.__fill()
        if encrypt_password is True:
            self.password = Encrypt.encrypt_password(self.password, self.name)

    def __str__(self):
        txt = f"Uuid: {self.uuid}, Name: {self.name}, Password: {self.password}, Email: {self.email}"
        return txt
