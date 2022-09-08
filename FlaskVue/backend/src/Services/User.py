import json
from Utils.DB import DB
from Utils.JSON import JSON
from Data.User import User as UserModel

class User:
    
    def __init__(self):
        self.db = DB()

    def __parse_from_dict(self, res):
        return UserModel(
            name=res["name"] if "name" in res else "",
            password=res["password"] if "password" in res else "",
            uuid=res["uuid"] if "uuid" in res else ""
        )

    def __check_user(self, user):
        if user.name == "" or user.password == "":
            return False
        return True

    def parse(self, data):
        if type(data) == dict:
            return self.__parse_from_dict(data)
        elif type(data) == list and len(data) > 0 and type(data[0]) == dict:
            return [self.__parse_from_dict(i) for i in data]
        elif type(data) == list:
            return []
        else:
            return {}

    def get(self, name=None):
        if name is not None:
            req = f"select * from User where name = '{name}'"
            res = self.db.get(req)
            if len(res) == 0:
                return {}
            return self.parse(res[0])
        req = "select * from User";
        res = self.db.get(req)
        users = self.parse(res)
        return users

    def create(self, data):
        user = self.parse(data)
        if not self.__check_user(user):
            return "Invalid user", 400
        req = f"insert into User (name, password) values ('{user.name}', '{user.password}')"
        res = self.db.post(req)
        if res:
            return self.get(name=user.name), 201
        else:
            return "Error", 500

    def login(self, data):
        user = self.parse(data)
        if not self.__check_user(user):
            return "Invalid user", 40
        req = f"select * from User where name = '{user.name}' and password = '{user.password}'"
        res = self.db.get(req)
        if len(res) == 0:
            return "Invalid user", 400
        else:
            return self.parse(res[0]), 200
