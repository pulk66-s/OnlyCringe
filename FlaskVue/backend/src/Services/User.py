import json
from Utils.DB import DB
from Data.User import User as UserModel

class User:
    
    def __init__(self):
        self.db = DB()

    def __parse_from_dict(self, res):
        return UserModel(
            name=res["name"] if "name" in res else "",
            password=res["password"] if "password" in res else ""
        )

    def parse(self, data):
        print("data", data)
        if type(data) == dict:
            return self.__parse_from_dict(data)
        elif type(data) == list and len(data) > 0 and type(data[0]) == dict:
            return [self.__parse_from_dict(i) for i in data]
        elif type(data) == list:
            return []
        else:
            return {}

    def get(self):
        req = "select * from User";
        res = self.db.get(req)
        users = self.parse(res)
        return users