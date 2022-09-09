from lib.Object.User import User as UserObject
from global_func import row_to_user

UserObj = UserObject()

@when("Modifying the field {field} to {value} for user {user}")
def Modifying_the_field_to_for_user(context, field, value, user):
    assert UserObj.modify({"name": user}, {field: value})