from lib.Object.User import User as UserObject
from global_func import parse_field, row_to_user

UserObj = UserObject()

@given("Cleaning all user database")
def clean_user_database(context):
    UserObj.delete(None, "db")

@when("Creating users")
def create_new_user(context):
    for row in context.table:
        user = row_to_user(row)
        assert UserObj.create(user)

@then("The users should be created")
def check_user_created(context):
    for row in context.table:
        user = row_to_user(row)
        res = UserObj.get("db", user)
        assert res is not None
        for k in user.keys():
            assert user[k] == res[k]

@then("The users cannot be created")
def the_users_cannot_be_created(context):
    for row in context.table:
        user = row_to_user(row)
        assert not UserObj.create(user)

@then("The user {user} cannot have the field {field} filled with {value}")
def the_user_cannot_have_the_field_filled_with(context, user, field, value):
    user = UserObj.get(get_with="db", user={"name": user})
    assert user[field] != value

@then("The user {user} should have the field {field} filled with {value}")
def the_user_should_have_the_field_filled_with(context, user, field, value):
    user = UserObj.get(get_with="db", user={"name": user})
    assert user[field] == value