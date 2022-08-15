from Pages.UserConditionPage import UserConditionPage

@then("I should be on the User Condition page")
def I_should_be_on_the_page(context):
    userConditionPage = UserConditionPage(None)
    assert context.curr_page.is_current_page(userConditionPage)