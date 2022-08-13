from lib.Object.User import User as UserObject
from global_func import row_to_user

UserObj = UserObject()

@then("I should be able to login with")
def I_should_be_able_to_login_with(context):
    for row in context.table:
        user = row_to_user(row)
        assert UserObj.login(user)