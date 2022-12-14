import json
from Utils.DB import DB
from Utils.JSON import JSON
from Data.User import User as UserModel
from Utils.Encrypt import Encrypt
from flask import request

class User:
    
    def __init__(self):
        self.db = DB()

    def __parse_from_dict(self, res):
        return UserModel(
            name=res["name"] if "name" in res else "",
            password=res["password"] if "password" in res else "",
            uuid=res["uuid"] if "uuid" in res else "",
            email=res["email"] if "email" in res else ""
        )

    def __check_user(self, user):
        fields = ["name", "password", "email"]
        for field in fields:
            if user.__dict__[field] == "":
                return False
        return True

    def __check_owner(self, user):
        header = request.headers["Authorization"]
        if Encrypt.encrypt_user(user) != header:
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

    def get(self, name=None, user=None, uuid=None):
        if uuid is not None:
            req = f"select * from User where uuid = '{uuid}'"
            res = self.db.get(req)
            if len(res) > 0:
                return self.parse(res)[0]
            else:
                return {}
        if name is not None:
            req = f"select * from User where name = '{name}'"
            res = self.db.get(req)
            if len(res) == 0:
                return {}
            return self.parse(res[0])
        if user is not None:
            vals = user.__dict__.items()
            req = "select * from User where " + " and ".join([f"{k} = '{v}'" for k, v in vals if v != ""])
            res = self.db.get(req)
            if len(res) == 0:
                return {}
            return self.parse(res[0])
        req = "select * from User";
        res = self.db.get(req)
        return self.parse(res)

    def create(self, data):
        user = self.parse(data)
        if not self.__check_user(user):
            raise Exception("Invalid user, 400")
        req = f"insert into User (name, password, email) values ('{user.name}', '{user.password}', '{user.email}')"
        res = self.db.post(req)
        if res:
            return self.get(name=user.name), 201
        else:
            raise Exception("User already exists, 400")

    def login(self, data):
        user = self.parse(data)
        if (user.name == "" and user.email == "") or user.password == "":
            raise Exception("Invalid user, 400")
        res = self.get(user=user)
        if res == {}:
            raise Exception("User not found, 400")
        else:
            return res, 200

    def delete(self, name=None):
        if name is not None:
            user = self.get(name=name)
            if user == {}:
                raise Exception("User does not exist, 400")
            if not self.__check_owner(user):
                raise Exception("You are not the owner of the account..., 400")
            req = f"delete from User where name='{name}'"
            if self.db.post(req) is True:
                return "Deleted", 200
            raise Exception("ServerError, 500")
        raise Exception("Cannot delete all users..., 400")

    def update(self, data, name=None):
        if name is not None:
            user = self.get(name=name)
            if user == {}:
                raise Exception("User does not exist, 400")
            if not self.__check_owner(user):
                raise Exception("You are not the owner of the account..., 400")
            fields = " and ".join([f"{k}='{v}'"for k, v in data.items() if k != "uuid"])
            req = f"update User set {fields} where name='{name}'"
            if self.db.post(req) is True:
                return self.get(name=data["name"] if "name" in data else name)
            raise Exception("ServerError, 500")
        raise Exception("Cannot update all users, 400")