from Lib.Global import parse_field
from Lib.Object import Object

class UserObject(Object):

    def __init__(self):
        super().__init__()

    def row_to_user(self, row):
        obj = {
            "uuid": parse_field(row, "uuid"),
            "name": parse_field(row, "name"),
            "description": parse_field(row, "description"),
            "password": parse_field(row, "password"),
            "email": parse_field(row, "email"),
            "archived": parse_field(row, "archived"),
            "creation_date": parse_field(row, "creation_date"),
            "verified": parse_field(row, "verified"),
            "role": parse_field(row, "role"),
        }
        if obj["role"] == "":
            obj["role"] = "USER"
        obj = {k: v for k, v in obj.items() if v is not None and v != ""}
        return obj

    def create(self, user, create_with="api"):
        if create_with == "api":
            res = self.api.post("/user", user)
            return res.status_code == 202
        else:
            raise ValueError(f"Create with {create_with} is not supported")

    def delete(self, delete_with="api"):
        if delete_with == "db":
            self.database.post("delete from User where 1=1")
            return True
        else:
            raise ValueError(f"Delete with {delete_with} is not supported")