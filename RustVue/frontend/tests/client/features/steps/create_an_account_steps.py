from Pages.CreateAnAccountPage import CreateAnAccountPage
from Drivers.Browser.GoogleChrome import GoogleChrome

@given("Opening the Create An Account page with {browser}")
def Opening_the_login_page_with_browser(context, browser):
    if browser == "GoogleChrome":
        context.createAnAccountPage = CreateAnAccountPage(GoogleChrome)
    else:
        raise ValueError(f"Browser {browser} is not supported")
    context.curr_page = context.createAnAccountPage
    context.createAnAccountPage.go()

@when("I fill the creating user form with")
def I_fill_the_creating_user_form_with(context):
    # The try catch are needed because context.table have no len...
    try:
        context.table[0]
    except IndexError:
        raise ValueError("No data to fill")
    try:
        context.table[1]
        raise ValueError("More than one row of data to fill")
    except IndexError:
        pass
    context.createAnAccountPage.fill_form(context.table[0])

@when("I click on the Create Account button")
def I_click_the_Create_An_Account_button(context):
    context.createAnAccountPage.click_create_an_account_button()

@when("I click on the create account Home page button")
def I_click_the_create_account_Home_page_button(context):
    context.createAnAccountPage.click_home_button()

@then("I should see the account created message")
def I_should_see_the_account_created_message(context):
    assert context.createAnAccountPage.is_success_message()

@then("I should be on the Create An Account page")
def I_should_be_on_the_create_an_account_page(context):
    createAnAccountPage = CreateAnAccountPage(None)
    assert context.curr_page.is_current_page(createAnAccountPage)

@then("I should see the account not created message")
def I_should_see_the_account_not_created_message(context):
    assert context.createAnAccountPage.is_error_message()

@then("I can not create user with form")
def I_should_not_create_user_with_form(context):
    for row in context.table:
        context.createAnAccountPage.fill_form(row)
        context.createAnAccountPage.click_create_an_account_button()
        assert context.createAnAccountPage.is_error_message()
        context.createAnAccountPage.clear_form()

@then("I can create user with form")
def I_should_create_user_with_form(context):
    for row in context.table:
        context.createAnAccountPage.fill_form(row)
        context.createAnAccountPage.click_create_an_account_button()
        context.curr_page.wait(0.3)
        assert context.createAnAccountPage.is_success_message()
        context.createAnAccountPage.clear_form()
