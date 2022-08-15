from Lib.User import UserObject

userObj = UserObject()

@given("Creating users")
def creating_users(context):
    for row in context.table:
        user = userObj.row_to_user(row)
        assert userObj.create(user)

@given("Deleting all users")
def deleting_all_users(context):
    assert userObj.delete(delete_with="db")
