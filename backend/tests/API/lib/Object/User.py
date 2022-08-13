from lib.Utils.API import API
from lib.Utils.Database import Database

class User:

    def __parse_user(self, row):
        obj = {
            "uuid": row[0],
            "name": row[1],
            "description": row[2],
            "password": row[3],
            "email": row[4],
            "archived": row[5],
            "creation_date": row[6],
            "verified": row[7],
            "role": row[8],
        }
        return obj

    def __init__(self):
        self.api = API("localhost", 8901)
        self.db = Database("localhost", "OCUser", "ççOCUs3r_p@ssw0rd__", "OnlyCringes")

    def create(self, user, create_with="api"):
        if create_with == "api":
            res = self.api.post("/api/user", user)
            return res.status_code == 202
        else:
            return False

    def get(self, get_with="api", user=None):
        if get_with == "api":
            return None
        elif get_with == "db":
            if user is not None:
                if "name" in user.keys():
                    name = user["name"]
                    req = f"select * from User where name='{name}'"
                    res = self.db.get(req)
                    if res == []:
                        return None
                    else:
                        return self.__parse_user(self.db.get(req)[0])
                else:
                    return None
            else:
                return None
        else:
            return None

    def delete(self, user=None, delete_with="api"):
        if delete_with == "api":
            if user is None:
                res = self.api.delete("/api/user")
                return res.status_code == 202
            else:
                if "name" in user.keys():
                    res = self.api.delete(f"/api/user/{user['name']}")
                    return res.status_code == 202
                elif "uuid" in user.keys():
                    res = self.api.delete(f"/api/user/{user['uuid']}")
                    return res.status_code == 202
        elif delete_with == "db":
            if user is None:
                req = "delete from User where 1=1"
                self.db.post(req)
                return True
            else:
                return False
        else:
            return False

    def login(self, user):
        res = self.api.post("/api/user/jwt/login", user)
        return res.status_code == 202

    def modify(self, user_to_modify, user_info):
        res = None
        if "name" in user_to_modify.keys():
            req = f"/api/user/{user_to_modify['name']}"
            res = self.api.put(req, user_info)
        elif "uuid" in user_to_modify.keys():
            req = f"/api/user/{user_to_modify['uuid']}"
            res = self.api.put(req, user_info)
        else:
            return False
        return res.status_code == 202