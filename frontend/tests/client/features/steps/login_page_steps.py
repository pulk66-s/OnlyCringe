from Pages.LoginPage import LoginPage
from Drivers.Browser.GoogleChrome import *

@given("Opening the login page with {browser}")
def Opening_the_login_page_with_browser(context, browser):
    if browser == "GoogleChrome":
        context.loginPage = LoginPage(GoogleChrome)
    else:
        raise ValueError(f"Browser {browser} is not supported")
    context.curr_page = context.loginPage
    context.loginPage.go()

@when("I fill login field with {login}")
def I_fill_login_field_with_login(context, login):
    context.loginPage.fill_login(login)

@when("I fill password field with {password}")
def I_fill_password_field_with_password(context, password):
    context.loginPage.fill_password(password)

@when("I click on the login button")
def I_click_on_login_button(context):
    context.loginPage.click_login_button()

@when("I click on the User Condition link")
def I_click_on_the_User_Condition_link(context):
    context.loginPage.click_user_condition()

@when("I click on the Create an account link")
def I_click_on_the_Create_an_account_link(context):
    context.loginPage.click_create_an_account()