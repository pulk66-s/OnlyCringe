from lib.Object.User import User as UserObject
from global_func import row_to_user

UserObj = UserObject()

@then("I should being able to delete the users")
def I_should_be_able_to_delete_the_users(context):
    for row in context.table:
        user = row_to_user(row)
        assert UserObj.delete(user=user)

@then("I should not being able to delete the users")
def I_should_not_being_able_to_delete_the_users(context):
    for row in context.table:
        user = row_to_user(row)
        assert not UserObj.delete(user=user)

@then("The user should be archived")
def The_user_should_be_archived(context):
    for row in context.table:
        user = row_to_user(row)
        db_user = UserObj.get(get_with="db", )